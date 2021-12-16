use std::{
    ffi::CString,
    fs::File,
    io::{self, Read, Seek, Write},
    mem,
    net::{SocketAddr, TcpListener, TcpStream},
    os::unix::io::{AsRawFd, FromRawFd},
    time::Instant,
};
use structopt::StructOpt;

const TCP_BLOCKSIZE: usize = 128 * 1024;

mod opts;
use opts::*;

mod error;
use error::Error;

mod util;
use util::wrap_io_err;

mod bindings;

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Server(server_opts) => server(server_opts).unwrap(),
        Opt::Client(client_opts) => client(client_opts).unwrap(),
    }
}

fn print_status(
    json: bool,
    bytes_written: usize,
    interval: u64,
    id: usize,
    retransmits: Option<u32>,
) {
    if json {
        return;
    }

    let transfer = bytesize::to_string(bytes_written as u64, false);
    let bandwidth = bytesize::to_string((bytes_written * 8) as u64 / interval, false);
    if let Some(retrans) = retransmits {
        println!(
            "[{:>3}] transfer= {} bandwidth= {}it/sec retransmits= {}",
            id, transfer, bandwidth, retrans
        );
    } else {
        println!(
            "[{:>3}] transfer= {} bandwidth= {}it/sec",
            id, transfer, bandwidth
        );
    }
}

// fn print_summary(bytes_written: usize, time: u64, retransmits: Option<u32>) {
fn print_summary(json: bool, summary: Summary) {
    if json {
        println!("{}", serde_json::to_string(&summary).unwrap());
        return;
    }

    if summary.time == 0 {
        println!("No summary available");
        return;
    };
    let transfer = bytesize::to_string(summary.bytes_written as u64, false);
    let bandwidth = bytesize::to_string((summary.bytes_written * 8) as u64 / summary.time, false);

    println!("-------");
    if let Some(retrans) = summary.retransmits {
        println!(
            "[SUM] time= {} transfer= {} bandwidth= {}it/sec retransmits= {}\n",
            summary.time, transfer, bandwidth, retrans
        );
    } else {
        println!(
            "[SUM] time= {} transfer= {} bandwidth= {}it/sec\n",
            summary.time, transfer, bandwidth
        );
    }
}

/// startup the client
fn client(opts: ClientOpts) -> Result<(), Error> {
    let mut addr = SocketAddr::new(opts.host, opts.port);

    match &mut addr {
        SocketAddr::V6(addr6) => {
            if let Some(interface) = &opts.interface {
                let if_name = CString::new(interface.as_str()).unwrap();
                let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
                addr6.set_scope_id(if_index);
            }
        }
        _ => {}
    }

    // create the control stream
    let mut control_stream = TcpStream::connect(addr).unwrap();

    // send client hello to control stream;
    let client_hello = ClientHello::from(&opts);

    let control = ControlMessage::ClientHello(client_hello.clone());
    control_stream.write(&bincode::serialize(&control)?)?;

    let control: ControlMessage = bincode::deserialize_from(&mut control_stream).unwrap();
    match control {
        ControlMessage::ServerHello(server_hello) => {
            eprintln!(
                "received server hello: data_port= {}",
                server_hello.data_port
            );
            let data_addr = {
                let mut tmp = addr.clone();
                tmp.set_port(server_hello.data_port);
                tmp
            };
            let mut data_stream = create_client_socket(data_addr, &client_hello)?;
            if opts.reversed {
                receiver(&mut data_stream, client_hello)?;
            } else {
                sender(&mut data_stream, client_hello)?;
            }
        }
        _ => return Err(Error::Protocol("unexpected control message")),
    }

    Ok(())
}

/// startup the server
fn server(opts: ServerOpts) -> Result<(), Error> {
    let mut addr = SocketAddr::from((opts.bind, opts.port));

    match &mut addr {
        SocketAddr::V6(addr6) => {
            if let Some(bind_dev) = &opts.bind_dev {
                let if_name = CString::new(bind_dev.as_str()).unwrap();
                let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
                addr6.set_scope_id(if_index);
            }
        }
        _ => {}
    }

    let listener = TcpListener::bind(addr)?;

    eprintln!("Server listening on {}:{}", addr.ip(), addr.port());

    loop {
        let (control_stream, client_addr) = listener.accept()?;

        match handle_client(control_stream, client_addr, addr, &opts) {
            Err(e) => eprintln!("Error handling client: {:?}", e),
            _ => {}
        }

        if opts.single {
            break;
        }
    }
    Ok(())
}

fn handle_client(
    mut control_stream: TcpStream,
    client_addr: SocketAddr,
    server_addr: SocketAddr,
    opts: &ServerOpts,
) -> Result<(), Error> {
    eprintln!("new connection from: {:?}", client_addr);

    let control: ControlMessage = bincode::deserialize_from(&mut control_stream).unwrap();

    let client_hello = match control {
        ControlMessage::ClientHello(hello) => hello,
        _ => return Err(Error::Protocol("unexpected control message")),
    };
    eprintln!(
        "received client configuration= {}",
        serde_json::to_string(&client_hello).unwrap()
    );

    // create data stream with random port
    let data_addr = {
        let mut tmp = server_addr.clone();
        tmp.set_port(opts.data_port);
        tmp
    };
    let data_socket = create_server_socket(data_addr, &client_hello)?;

    // send server hello containing socket address of data socket message to client
    let server_hello = ServerHello {
        data_port: data_socket.local_addr()?.port(),
    };
    let control = ControlMessage::ServerHello(server_hello);
    control_stream.write(&bincode::serialize(&control)?)?;

    let (mut data_stream, _client_addr) = data_socket.accept()?;
    // TODO: maybe check if client address is the same as before

    if client_hello.reversed {
        sender(&mut data_stream, client_hello)
    } else {
        receiver(&mut data_stream, client_hello)
    }
}

/// Use the given socket as the sender
/// this function will only write to the socket
/// The client_hello will contain all the necessary information on how to write exactly
/// eg. zerocopy and time
fn sender<S>(stream: &mut S, client_hello: ClientHello) -> Result<(), Error>
where
    S: Write + std::os::unix::io::AsRawFd,
{
    // create a new temporary file that holds random data
    // if zerocopy is enabled this file will be sent to the receiver with sendfile()
    // otherwise it will be read into a buffer and sent via read()
    let mut buf_fd = create_temp_file(TCP_BLOCKSIZE)?;
    let mut buf = vec![0; TCP_BLOCKSIZE];
    if !client_hello.zerocopy {
        buf_fd.read(&mut buf)?;
    }

    // written contains the total bytes written in each interval
    let mut written: Vec<usize> = vec![0; client_hello.time as usize];

    // Save starting time to determine how long to run the sending for
    let mut last_interval = 0;
    let mut total_retransmits = 0;
    let start = Instant::now();

    loop {
        // save the elapsed time since the start in a variable since this value could change by the
        // time the check runs if the loop should finish
        let elapsed = start.elapsed().as_secs();
        let current_interval = elapsed as usize;

        if last_interval != current_interval {
            // read the tcp_info socket option. This contains information about the current
            // TcpStream, specifically the total retransmits that happened for this stream.
            let tcp_info: bindings::tcp_info =
                unsafe { util::getsockopt(stream.as_raw_fd(), libc::IPPROTO_TCP, libc::TCP_INFO)? };

            // calculate the retransmits for the current interval by taking the difference of the
            // total retransmits and the retransmits at the last check
            let current_retransmits = tcp_info.tcpi_total_retrans - total_retransmits;
            total_retransmits = tcp_info.tcpi_total_retrans;

            print_status(
                client_hello.json,
                written[current_interval - 1],
                1,
                current_interval,
                Some(current_retransmits),
            );
            last_interval = current_interval;
        }

        // break out of the loop if the user provided time has elapsed
        if elapsed >= client_hello.time {
            break;
        }

        let n = if client_hello.zerocopy {
            // if the client selected the zerocopy option use sendfile() to send the previously
            // created buffer file to the receiver.
            // This is much more efficient because we reduce the amount of syscalls and cpu cycles
            let res =
                unsafe { util::sendfile(stream.as_raw_fd(), buf_fd.as_raw_fd(), TCP_BLOCKSIZE)? };

            // rewind the cursor to start reading at the beginning of the file in the next
            // iteration of the loop
            buf_fd.rewind()?;

            res
        } else {
            // simply write the buffer to the stream
            stream.write(&mut buf)?
        };

        // update total bytes written for the current interval
        *written.get_mut(current_interval).unwrap() += n;
    }

    print_summary(
        client_hello.json,
        Summary {
            bytes_written: written.iter().sum(),
            time: start.elapsed().as_secs(),
            retransmits: Some(total_retransmits),
        },
    );
    Ok(())
}

/// use the given socket as the receiver
/// only read from the socket
fn receiver<S>(socket: &mut S, client_hello: ClientHello) -> Result<(), Error>
where
    S: Read,
{
    // create a receive buffer
    let mut buf = vec![0; client_hello.buffer_size];
    let mut written: Vec<usize> = vec![0; client_hello.time as usize];

    let start = Instant::now();
    let mut before = 0;

    loop {
        let current_interval = start.elapsed().as_secs() as usize;
        if before != current_interval {
            print_status(
                client_hello.json,
                written[current_interval - 1],
                1,
                current_interval,
                None,
            );
            before = current_interval;
        }

        match socket.read(&mut buf)? {
            0 => break,
            n => {
                written.get_mut(current_interval).map(|w| *w += n);
            }
        }
    }

    print_summary(
        client_hello.json,
        Summary {
            bytes_written: written.iter().sum(),
            time: start.elapsed().as_secs(),
            retransmits: None,
        },
    );
    Ok(())
}

unsafe fn create_raw_socket(addr: SocketAddr, opts: &ClientHello) -> Result<i32, io::Error> {
    // determine socket family
    let fam = match addr {
        SocketAddr::V4(_) => libc::AF_INET,
        SocketAddr::V6(_) => libc::AF_INET6,
    };

    // create a raw socket
    let fd = wrap_io_err(libc::socket(fam, libc::SOCK_STREAM, 0))?;

    // set the maximum segment size for this socket
    if let Some(mss) = opts.mss {
        wrap_io_err(util::setsockopt(
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_MAXSEG,
            mss,
        ))?;
    }

    Ok(fd)
}

/// create a client socket from a raw fd
/// this is used to set socket options before connecting
fn create_client_socket(addr: SocketAddr, opts: &ClientHello) -> Result<TcpStream, io::Error> {
    let fd = unsafe { create_raw_socket(addr, opts)? };

    let (addrp, len) = unsafe { addr_into_inner(&addr) };

    wrap_io_err(unsafe { libc::connect(fd, addrp, len) })?;

    Ok(unsafe { TcpStream::from_raw_fd(fd) })
}

/// create a server socket from a raw fd
/// this is used to set socket options before connecting
fn create_server_socket(addr: SocketAddr, opts: &ClientHello) -> Result<TcpListener, io::Error> {
    let fd = unsafe { create_raw_socket(addr, opts)? };

    // SAFETY: addr lives for the whole function
    let (addrp, len) = unsafe { addr_into_inner(&addr) };

    wrap_io_err(unsafe {
        util::setsockopt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1 as libc::c_int)
    })?;

    wrap_io_err(unsafe { libc::bind(fd, addrp, len as _) }).unwrap();
    wrap_io_err(unsafe { libc::listen(fd, 128) })?;

    Ok(unsafe { TcpListener::from_raw_fd(fd) })
}

/// shamelessly stolen from the rust private std lib
/// (they could have just made the module public)
///
/// SAFETY: addr has to outlive the function so this has to be a &SocketAddr
unsafe fn addr_into_inner(addr: &SocketAddr) -> (*const libc::sockaddr, libc::socklen_t) {
    match addr {
        SocketAddr::V4(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as libc::socklen_t,
        ),
        SocketAddr::V6(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as libc::socklen_t,
        ),
    }
}

fn create_temp_file(blksize: usize) -> io::Result<File> {
    let template = std::env::temp_dir().join("iperf.XXXXXX");
    let template = CString::new(template.to_str().unwrap()).unwrap();
    let fd = wrap_io_err(unsafe { libc::mkstemp(template.as_ptr() as _) }).unwrap();
    let mut outfile = unsafe { File::from_raw_fd(fd) };

    let mut rand = File::open("/dev/urandom").unwrap();
    let mut buf = vec![0; blksize];
    rand.read(&mut buf)?;
    outfile.write(&buf)?;
    outfile.rewind().unwrap();

    Ok(outfile)
}
