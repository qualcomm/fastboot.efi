#!/bin/sh -e

rm -rf out/
mkdir -p out/

systemd-repart out/EFIESP-ufs.img --empty=create --size=512M --definitions=repart.d --root=$PWD --sector-size=4096
systemd-repart out/EFIESP-nvme-emmc.img --empty=create --size=512M --definitions=repart.d --root=$PWD
