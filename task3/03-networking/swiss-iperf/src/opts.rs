use serde::{Deserialize, Serialize};
use std::{convert::From, net::IpAddr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// server port of control channel
    #[structopt(short, long, default_value = "5202")]
    pub port: u16,

    /// Print output as JSON
    #[structopt(short, long)]
    pub json: bool,
}

#[derive(Debug, StructOpt)]
pub struct ServerOpts {
    /// server port of control channel
    #[structopt(short, long, default_value = "5202")]
    pub port: u16,

    /// server port of data channel (random if unassigned)
    #[structopt(long, default_value = "0")]
    pub data_port: u16,

    /// bind to interface address
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub bind: IpAddr,

    /// accept only a single client
    #[structopt(short = "1", long)]
    pub single: bool,

    /// network interface to bind to (eg. eth0)
    #[structopt(long)]
    pub bind_dev: Option<String>,

    /// Print output as JSON
    #[structopt(short, long)]
    pub json: bool,

    #[structopt(flatten)]
    pub common_opts: CommonOpts,
}

#[derive(Debug, StructOpt)]
pub struct ClientOpts {
    /// Host to connect to
    pub host: IpAddr,

    /// Server sends data to client
    #[structopt(short = "R", long)]
    pub reversed: bool,

    /// Time to run the benchmark (in seconds)
    #[structopt(short, long, default_value = "10")]
    pub time: u64,

    /// Network interface to bind to (eg. eth0)
    /// usually appended with a % to the end of the ip6 but Rust does not support that
    #[structopt(short, long)]
    pub interface: Option<String>,

    /// Maximum segment size
    #[structopt(short = "M", long = "set-mss")]
    pub mss: Option<i32>,

    /// Tcp send and receive buffer size
    #[structopt(short = "w", long)]
    pub window: Option<usize>,

    /// Use a zerocopy way of sending data
    #[structopt(short = "Z", long)]
    pub zerocopy: bool,

    #[structopt(flatten)]
    pub common_opts: CommonOpts,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "swiss-iperf", about = "An iperf clone")]
pub enum Opt {
    #[structopt(alias("s"))]
    Server(ServerOpts),
    #[structopt(alias("c"))]
    Client(ClientOpts),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ClientHello {
    pub time: u64,
    pub mss: Option<i32>,
    pub reversed: bool,
    pub window: Option<usize>,
    pub zerocopy: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ServerHello {
    pub data_port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ControlMessage {
    ClientHello(ClientHello),
    ServerHello(ServerHello),
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub bytes_written: usize,
    pub time: u64,
    pub retransmits: Option<u32>,
}

impl From<&ClientOpts> for ClientHello {
    fn from(opts: &ClientOpts) -> Self {
        ClientHello {
            time: opts.time,
            mss: opts.mss,
            reversed: opts.reversed,
            window: opts.window,
            zerocopy: opts.zerocopy,
        }
    }
}
