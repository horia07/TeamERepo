#!/bin/sh
nix-env -iA nixos.gnuplot
nix-env -iA nixos.btrfs-progs
nix-env -iA nixos.fio


sudo mkdir /mnt/teame-ext4
sudo mkdir /mnt/teame-btrfs

mkdir results

bash ext4benchmark.sh
bash btrfsbenchmark.sh

bash gen_plot

