use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use structopt::StructOpt;

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
    #[structopt(short, long)]
    pub single: bool,

    /// network interface to bind to (eg. eth0)
    #[structopt(long)]
    pub bind_dev: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct ClientOpts {
    /// Print output as JSON
    #[structopt(short, long)]
    pub json: bool,

    /// Host to connect to
    pub host: IpAddr,

    /// Server port
    #[structopt(short, long, default_value = "5202")]
    pub port: u16,

    /// Server sends data to client
    #[structopt(short = "R", long)]
    pub reversed: bool,

    /// Time to run the benchmark (in seconds)
    #[structopt(short, long)]
    pub time: u64,

    /// Network interface to bind to (eg. eth0)
    /// normally appended with a % to the end of the ip6 but Rust does not support that
    #[structopt(short, long)]
    pub interface: Option<String>,

    /// Maximum segment size
    #[structopt(short = "M", long)]
    pub mss: Option<i32>,

    /// Tcp buffer size
    #[structopt(long, default_value = "4096")]
    pub buffer_size: usize,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "swiss-iperf", about = "An iperf clone")]
pub enum Opt {
    Server(ServerOpts),
    Client(ClientOpts),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ClientHello {
    pub time: u64,
    pub mss: Option<i32>,
    pub reversed: bool,
    pub buffer_size: usize,
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
