# docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.57.0 cargo build --release
nix-shell -p cargo --command "cargo build --release"
mkdir -p bin
ln -fs ../target/release/swiss-iperf bin/swiss-iperf
