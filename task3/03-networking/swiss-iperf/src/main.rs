use std::{
    ffi::CString,
    io::{self, Read, Write},
    mem,
    net::{SocketAddr, TcpListener, TcpStream},
    time::Instant,
};
use structopt::StructOpt;

mod opts;
use opts::*;

mod error;
use error::Error;

mod util;

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Server(server_opts) => server(server_opts).unwrap(),
        Opt::Client(client_opts) => client(client_opts).unwrap(),
    }
}

fn print_status(bytes_written: usize, interval: u64, id: usize) {
    let transfer = bytesize::to_string(bytes_written as u64, false);
    let bandwidth = bytesize::to_string((bytes_written * 8) as u64 / interval, false);
    println!(
        "[{}] transfer= {} bandwidth= {}it/sec",
        id, transfer, bandwidth
    );
}

fn print_summary(bytes_written: usize, time: u64) {
    let transfer = bytesize::to_string(bytes_written as u64, false);
    let bandwidth = bytesize::to_string((bytes_written * 8) as u64 / time, false);

    println!("-------");
    println!(
        "[*] transfer= {}, bandwidth= {}it/sec\n",
        transfer, bandwidth
    );
}

/// startup the server
fn client(opts: ClientOpts) -> Result<(), Error> {
    let mut addr = SocketAddr::new(opts.host, opts.port);

    match &mut addr {
        SocketAddr::V6(addr6) => {
            if let Some(bind_dev) = &opts.interface {
                let if_name = CString::new(bind_dev.as_str()).unwrap();
                let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
                addr6.set_scope_id(if_index);
            }
        }
        _ => {}
    }

    // create the control stream
    let mut control_stream = TcpStream::connect(addr).unwrap();

    // send client hello to control stream;
    let client_hello = ClientHello {
        time: opts.time,
        reversed: opts.reversed,
        mss: opts.mss,
        buffer_size: opts.buffer_size,
    };
    let control = ControlMessage::ClientHello(client_hello.clone());
    control_stream.write(&bincode::serialize(&control)?)?;

    let control: ControlMessage = bincode::deserialize_from(&mut control_stream).unwrap();
    match control {
        ControlMessage::ServerHello(server_hello) => {
            println!(
                "received server hello: data_port= {}",
                server_hello.data_port
            );
            let data_addr = {
                let mut tmp = addr.clone();
                tmp.set_port(server_hello.data_port);
                tmp
            };
            let mut data_stream = unsafe { create_client_socket(data_addr, &opts)? };
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
            if let Some(bind_dev) = opts.bind_dev {
                let if_name = CString::new(bind_dev).unwrap();
                let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
                addr6.set_scope_id(if_index);
            }
        }
        _ => {}
    }

    let listener = TcpListener::bind(addr)?;

    println!("Server listening on {}:{}", addr.ip(), addr.port());

    loop {
        let (mut control_stream, client_addr) = listener.accept()?;

        println!("new connection from: {:?}", client_addr);

        let control: ControlMessage = bincode::deserialize_from(&mut control_stream).unwrap();

        let client_hello = match control {
            ControlMessage::ClientHello(hello) => hello,
            _ => return Err(Error::Protocol("unexpected control message")),
        };
        println!(
            "received client configuration= {}",
            serde_json::to_string(&client_hello).unwrap()
        );

        // create data stream with random port
        let data_addr = {
            let mut tmp = addr.clone();
            tmp.set_port(opts.data_port);
            tmp
        };
        let data_socket = TcpListener::bind(data_addr)?;

        // send server hello containing socket address of data socket message to client
        let server_hello = ServerHello {
            data_port: data_socket.local_addr()?.port(),
        };
        let control = ControlMessage::ServerHello(server_hello);
        control_stream.write(&bincode::serialize(&control)?)?;

        let (mut data_stream, _client_addr) = data_socket.accept()?;
        // TODO: maybe check if client address is the same as before

        if client_hello.reversed {
            sender(&mut data_stream, client_hello)?;
        } else {
            receiver(&mut data_stream, client_hello)?;
        }

        if opts.single {
            break;
        }
    }
    Ok(())
}

/// use the given socket as the sender
/// only write to the socket
fn sender<S>(stream: &mut S, client_hello: ClientHello) -> Result<(), io::Error>
where
    S: Write,
{
    let start = Instant::now();
    let mut before = 0;

    let mut buf = vec![0; client_hello.buffer_size];
    util::fill_random(&mut buf);
    let mut written: Vec<usize> = vec![0; client_hello.time as usize];

    loop {
        let elapsed = start.elapsed().as_secs();
        let segment = elapsed as usize;

        if before != segment {
            print_status(written[segment - 1], 1, segment);

            before = segment;
        }

        if start.elapsed().as_secs() >= client_hello.time {
            break;
        }

        let n = stream.write(&mut buf).unwrap();
        *written.get_mut(segment).unwrap() += n;
    }

    print_summary(written.iter().sum(), start.elapsed().as_secs());
    Ok(())
}

/// use the given socket as the receiver
/// only read from the socket
fn receiver<S>(socket: &mut S, client_hello: ClientHello) -> Result<(), io::Error>
where
    S: Read,
{
    let start = Instant::now();
    let mut before = 0;

    let mut buf = vec![0; client_hello.buffer_size];
    let mut written: Vec<usize> = vec![0; client_hello.time as usize];

    loop {
        let segment = start.elapsed().as_secs() as usize;
        if before != segment {
            print_status(written[segment - 1], 1, segment);
            before = segment;
        }

        match socket.read(&mut buf)? {
            0 => break,
            n => {
                written.get_mut(segment).map(|w| *w += n);
            }
        }
    }

    print_summary(written.iter().sum(), start.elapsed().as_secs());
    Ok(())
}

/// create a client socket from a raw fd
/// this is used to set socket options before connecting
unsafe fn create_client_socket(
    addr: SocketAddr,
    opts: &ClientOpts,
) -> Result<TcpStream, io::Error> {
    let fam = match addr {
        SocketAddr::V4(_) => libc::AF_INET,
        SocketAddr::V6(_) => libc::AF_INET6,
    };

    let fd = libc::socket(fam, libc::SOCK_STREAM, 0);

    // set the maximum segment size for this socket
    if let Some(mss) = opts.mss {
        let res = util::setsockopt(fd, libc::TCP_MAXSEG, mss);
        assert_eq!(0, res, "could not set mss");
    }

    // shamelessly stolen from the rust private std lib
    // (they could have just made the module public)
    let (addrp, len) = match addr {
        SocketAddr::V4(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as libc::socklen_t,
        ),
        SocketAddr::V6(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as libc::socklen_t,
        ),
    };

    libc::connect(fd, addrp, len);

    use std::os::unix::io::FromRawFd;
    Ok(TcpStream::from_raw_fd(fd))
}
