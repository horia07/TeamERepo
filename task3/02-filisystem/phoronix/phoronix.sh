sudo mkdir /mnt/teame-ext4-phoronix
sudo mkfs.ext4 /dev/mapper/swissknife-teame
sudo mount /dev/mapper/swissknife-teame /mnt/teame-ext4-phoronix

docker build -t ph -f Dockerfile .
docker run --rm -it -v ${PWD}/results:/var/lib/phoronix-test-suite/test-results -v ${PWD}/suites:/var/lib/phoronix-test-suite/test-suites --mount type=bind,source=/mnt/teame-ext4-phoronix/,target=/user/src/app ph

sudo umount /dev/mapper/swissknife-teame
sudo rm -r /mnt/teame-ext4-phoronix
sudo blkdiscard -f /dev/mapper/swissknife-teame
