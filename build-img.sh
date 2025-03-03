#!/bin/sh -e

rm -f fastboot-ufs-disk.img
sudo systemd-repart fastboot-ufs-disk.img --empty=create --size=512M --sector-size=4096 --definitions=repart.d --root=$PWD
