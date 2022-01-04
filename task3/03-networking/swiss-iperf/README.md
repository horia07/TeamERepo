# swiss-iperf

An iperf3 clone written in Rust for the swissknife practical course.

# Features

- measure tcp bandwidth
- variable tcp send and receive buffer
- variable tcp maximum segment size
- reverse mode
- zerocopy mode

# Build

Depending on if you have cargo or nix installed run the appropriate build script

```console
bash build_nix.sh   # build with nix
bash build_cargo.sh # build with cargo
```


# Reproduction

## Setup
To reproduce the experiments you need to start an instance of both iperf3 and swiss-iperf in a different terminal each.
You can use the provided scripts to start a server.

```console
bash start_iperf3_server.sh
bash start_swiss_server.sh
```

Alternatively you can start the servers manually:
```console
iperf3 -s --bind fe80::e63d:1aff:fe72:f0%swissknife0
bin/swiss-iperf server --bind fe80::e63d:1aff:fe72:f0 --bind-dev swissknife0
```

## Execution
After that run the following command to reproduce the experiments
```console
bash reproduce.sh
```

This takes a while and generates a new data file and plot in the `result` directory.
