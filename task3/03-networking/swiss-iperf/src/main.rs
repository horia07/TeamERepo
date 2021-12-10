use serde::{Deserialize, Serialize};
use std::{
    ffi::CString,
    io::{Read, Write},
    net::{IpAddr, SocketAddr, SocketAddrV6, TcpListener, TcpStream},
    time::Instant,
};
use structopt::StructOpt;

const BUF_SIZE: usize = 4096;

fn fill_random(buf: &mut [u8]) {
    unsafe {
        libc::getrandom(buf.as_mut_ptr() as _, buf.len(), 0);
    }
}

#[derive(Debug, StructOpt)]
struct ServerOpts {
    /// server port
    #[structopt(default_value = "5202")]
    port: u16,

    /// bind to interface address
    #[structopt(long, default_value = "0.0.0.0")]
    bind: IpAddr,

    /// accept only a single client
    #[structopt(short, long)]
    single: bool,

    /// network interface to bind to (eg. eth0)
    #[structopt(long)]
    bind_dev: Option<String>,
}

#[derive(Debug, StructOpt)]
struct ClientOpts {
    /// host
    host: IpAddr,

    /// server port
    #[structopt(default_value = "5202")]
    port: u16,

    /// server sends data to clients
    #[structopt(short = "R", long)]
    reversed: bool,

    /// time to run the benchmark
    #[structopt(short, long)]
    time: u64,

    /// network interface to bind to (eg. eth0)
    /// normally appended with a % to the end of the ip6 but rust does not support this.
    #[structopt(long)]
    bind_dev: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "swiss-iperf", about = "An iperf clone")]
enum Opt {
    Server(ServerOpts),
    Client(ClientOpts),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct ClientHello {
    time: u64,
    reversed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum ControlMessage {
    ClientHello(ClientHello),
    ServerHello,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Server(server_opts) => server(server_opts),
        Opt::Client(client_opts) => client(client_opts),
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

fn client(opts: ClientOpts) {
    let addr = SocketAddr::new(opts.host, opts.port);

    if let SocketAddr::V6(mut addr6) = addr {
        if let Some(bind_dev) = opts.bind_dev {
            let if_name = CString::new(bind_dev).unwrap();
            let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
            addr6.set_scope_id(if_index);
        }
    }

    let mut stream = TcpStream::connect(addr).unwrap();

    let hello = ClientHello {
        time: opts.time,
        reversed: opts.reversed,
    };
    let control = ControlMessage::ClientHello(hello.clone());

    bincode::serialize_into(&mut stream, &control).unwrap();
    let control: ControlMessage = bincode::deserialize_from(&mut stream).unwrap();

    if control != ControlMessage::ServerHello {
        panic!("wrong control message");
    }

    if opts.reversed {
        receiver(&mut stream, hello.time);
    } else {
        sender(&mut stream, hello.time);
    }
}

fn server(opts: ServerOpts) {
    let addr = SocketAddr::from((opts.bind, opts.port));
    if let SocketAddr::V6(mut addr6) = addr {
        if let Some(bind_dev) = opts.bind_dev {
            let if_name = CString::new(bind_dev).unwrap();
            let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr() as _) };
            addr6.set_scope_id(if_index);
        }
    }

    let listener = TcpListener::bind(addr).expect("cound not bind");

    println!("Server listening on {}:{}", addr.ip(), addr.port());

    loop {
        let (mut stream, addr) = listener.accept().unwrap();

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

        bincode::serialize_into(&mut stream, &ControlMessage::ServerHello).unwrap();

        if hello.reversed {
            sender(&mut stream, hello.time);
        } else {
            receiver(&mut stream, hello.time);
        }

        if opts.single {
            break;
        }
    }
}

fn sender<S>(stream: &mut S, time: u64)
where
    S: Write,
{
    let start = Instant::now();
    let mut last_print = Instant::now();

    let mut buf = [0; BUF_SIZE];
    fill_random(&mut buf);

    let mut written: Vec<usize> = vec![0; time as usize];
    let mut segment: usize = 0;

    loop {
        if last_print.elapsed().as_secs() >= 1 {
            print_status(written[segment], 1, segment);

            last_print = Instant::now();
            segment += 1;
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
}

fn receiver<S>(socket: &mut S, time: u64)
where
    S: Read,
{
    let start = Instant::now();
    let mut last_print = Instant::now();

    let mut buf = [0; BUF_SIZE];

    let mut written: Vec<usize> = vec![0; time as usize];
    let mut segment: usize = 0;

    loop {
        if last_print.elapsed().as_secs() >= 1 {
            print_status(written[segment], 1, segment);
            last_print = Instant::now();
            segment += 1;
        }

        let n = socket.read(&mut buf).unwrap();

        if n == 0 {
            break;
        };

        *written.get_mut(segment).unwrap() += n;
    }

    let total_time = start.elapsed();

    let total_written: usize = written.iter().sum();
    let transfer = bytesize::to_string(total_written as u64, false);
    let bandwidth = bytesize::to_string((total_written * 8) as u64 / total_time.as_secs(), false);

    println!("-------");
    println!("[*] transfer= {}, bandwidth= {}it/sec", transfer, bandwidth);
}
