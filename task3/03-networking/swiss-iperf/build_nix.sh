nix-shell -p cargo --command "cargo build --release"
mkdir -p bin
ln -fs ../target/release/swiss-iperf bin/swiss-iperf
