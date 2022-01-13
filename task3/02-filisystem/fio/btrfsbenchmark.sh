#!/bin/sh

sudo mkfs.btrfs /dev/mapper/swissknife-teame
sudo mount /dev/mapper/swissknife-teame /mnt/teame-btrfs

sudo MOUNT='/mnt/teame-btrfs/data.txt' fio readrandom.fio
sudo MOUNT='/mnt/teame-btrfs/data.txt' fio writerandom.fio

mv rand_read_bw.log btrfsrand_read_bw.log
mv rand_write_bw.log btrfsrand_write_bw.log
mv btrfsrand_read_bw.log results
mv btrfsrand_write_bw.log results

sudo umount /dev/mapper/swissknife-teame
sudo rm -r /mnt/teame-btrfs
sudo blkdiscard -f /dev/mapper/swissknife-teame
