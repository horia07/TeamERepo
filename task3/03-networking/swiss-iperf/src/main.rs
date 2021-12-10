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

const BUF_SIZE: usize = 4096;

fn main() {
    let s = TcpListener::bind("0.0.0.0:0").unwrap();
    dbg!(s.local_addr());

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

/// startup the server
fn client(opts: ClientOpts) -> Result<(), Error> {
    let mut addr = SocketAddr::new(opts.host, opts.port);

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

    // let mut stream = TcpStream::connect(addr).unwrap();
    let mut stream = unsafe { create_client_socket(addr, &opts)? };

    if let Ok(addr) = stream.local_addr() {
        dbg!(addr);
    }

    let hello = ClientHello {
        time: opts.time,
        reversed: opts.reversed,
        mss: opts.mss,
    };
    let control = ControlMessage::ClientHello(hello.clone());

    bincode::serialize_into(&mut stream, &control).unwrap();
    let control: ControlMessage = bincode::deserialize_from(&mut stream).unwrap();

    if control != ControlMessage::ServerHello {
        panic!("wrong control message");
    }

    if opts.reversed {
        receiver(&mut stream, hello.time)?;
    } else {
        sender(&mut stream, hello.time)?;
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
        let (mut stream, addr) = listener.accept()?;

        println!("new connection from: {:?}", addr);

        let control: ControlMessage = bincode::deserialize_from(&mut stream).unwrap();

        let hello = match control {
            ControlMessage::ClientHello(hello) => hello,
            _ => panic!("wrong control message"),
        };
        println!(
            "received client configuration: time= {} reversed= {}",
            hello.time, hello.reversed
        );

        bincode::serialize_into(&mut stream, &ControlMessage::ServerHello)?;

        if hello.reversed {
            sender(&mut stream, hello.time)?;
        } else {
            receiver(&mut stream, hello.time)?;
        }

        if opts.single {
            break;
        }
    }
    Ok(())
}

/// use the given socket as the sender
/// only write to the socket
fn sender<S>(stream: &mut S, time: u64) -> Result<(), io::Error>
where
    S: Write,
{
    let start = Instant::now();
    let mut before = 0;

    let mut buf = [0; BUF_SIZE];
    util::fill_random(&mut buf);
    let mut written: Vec<usize> = vec![0; time as usize];

    loop {
        let elapsed = start.elapsed().as_secs();
        let segment = elapsed as usize;

        if before != segment {
            print_status(written[segment - 1], 1, segment);

            before = segment;
        }

        if start.elapsed().as_secs() >= time {
            break;
        }

        let n = stream.write(&mut buf).unwrap();
        *written.get_mut(segment).unwrap() += n;
    }

    let total_time = start.elapsed();

    let total_written: usize = written.iter().sum();
    let transfer = bytesize::to_string(total_written as u64, false);
    let bandwidth = bytesize::to_string((total_written * 8) as u64 / total_time.as_secs(), false);

    println!("-------");
    println!("[*] transfer= {}, bandwidth= {}it/sec", transfer, bandwidth);
    Ok(())
}

/// use the given socket as the receiver
/// only read from the socket
fn receiver<S>(socket: &mut S, time: u64) -> Result<(), io::Error>
where
    S: Read,
{
    let start = Instant::now();
    let mut before = 0;

    let mut buf = [0; BUF_SIZE];
    let mut written: Vec<usize> = vec![0; time as usize];

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

    let total_time = start.elapsed();

    let total_written: usize = written.iter().sum();
    let transfer = bytesize::to_string(total_written as u64, false);
    let bandwidth = bytesize::to_string((total_written * 8) as u64 / total_time.as_secs(), false);

    println!("-------");
    println!("[*] transfer= {}, bandwidth= {}it/sec", transfer, bandwidth);
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
