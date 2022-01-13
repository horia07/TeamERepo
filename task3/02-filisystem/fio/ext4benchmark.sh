#!/bin/sh

sudo mkfs.ext4 /dev/mapper/swissknife-teame
sudo mount /dev/mapper/swissknife-teame /mnt/teame-ext4

sudo MOUNT='/mnt/teame-ext4/data.txt' fio readrandom.fio
sudo MOUNT='/mnt/teame-ext4/data.txt' fio writerandom.fio

mv rand_read_bw.log ext4rand_read_bw.log
mv rand_write_bw.log ext4rand_write_bw.log
mv ext4rand_read_bw.log results
mv ext4rand_write_bw.log results

sudo umount /dev/mapper/swissknife-teame
sudo rm -r /mnt/teame-ext4
sudo blkdiscard -f /dev/mapper/swissknife-teame




