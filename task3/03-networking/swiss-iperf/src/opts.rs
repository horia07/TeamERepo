use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ServerOpts {
    /// server port of control channel
    #[structopt(default_value = "5202")]
    pub port: u16,

    /// server port of data channel (random if unassigned)
    #[structopt(default_value = "0")]
    pub data_port: u16,

    /// bind to interface address
    #[structopt(long, default_value = "0.0.0.0")]
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
    /// host
    pub host: IpAddr,

    /// server port
    #[structopt(default_value = "5202")]
    pub port: u16,

    /// server sends data to clients
    #[structopt(short = "R", long)]
    pub reversed: bool,

    /// time to run the benchmark
    #[structopt(short, long)]
    pub time: u64,

    /// network interface to bind to (eg. eth0)
    /// normally appended with a % to the end of the ip6 but rust does not support this.
    #[structopt(long)]
    pub bind_dev: Option<String>,

    /// maximum segment size
    #[structopt(short = "M", long)]
    pub mss: Option<i32>,
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
