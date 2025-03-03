# Fastboot EFI application

*fastboot.efi* is an implementation of a subset of the fastboot protocol as a
standalone EFI application, for the purpose of enabling workflows using
*fastboot boot* for e.g. rapid kernel development in environments that doesn't
already provide fastboot support. It also has a more optimistic memory
allocation policy, allowing for booting large images.

*fastboot.efi* supports booting abootimg v2 and PE32+ (EFI) images.

Supported fastboot commands are **boot**, **continue**, and **reboot**.

## Building

Use *rustup* to install the aarch64-unknown-uefi target. Then build using:

```
cargo build --target aarch64-unknown-uefi
```

## Deploying

The resulting *fastboot.efi* can be loaded by normal means of loading EFI
applications, such as chainloading from systemd-boot/grub or placed as
*\EFI\boot\bootaa64.efi*.

### Generating disk image

To further simplify bootstrapping processes of a device for development
purposes, *fastboot.efi* can be packaged in an EFI System Partition (esp),
wrapped by a GPT header using the included repart.d configuration and the
following command:

```
systemd-repart fastboot-nvme-disk.img --empty=create --size=64M --definitions=repart.d --root=$PWD
```
or:
```
systemd-repart fastboot-ufs-disk.img --empty=create --size=512M --sector-size=4096 --definitions=repart.d --root=$PWD
```

This file can be written straight to a NVMe (or UFS) device, which upon booting
automatically enters fastboot mode.

*rawprogram-nvme.xml* and *rawprogram-ufs.xml* are included for convenient
application with common flash tools.

## *fastboot boot* an UKI (or other EFI application)

The standard *fastboot* host tool, will upon invoking *fastboot boot* wrap the
provided non-abootimg file in an abootimg format before downloading and booting
it on the device.

By using a "non-standard" *fastboot* host tool, that downloads the file
provided as is, *fastboot.efi* allows booting arbitrary EFI applications, such
as Unified Kernel Images (UKI).

## Contribute

With the goal of providing a convenient development environment for upstream
work, please do contribute to both implementation and documentation by opening
a Pull Request. Issues can be used to track issues with the implementation,
documentation, and device-specific issues.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## License

Licensed under [BSD 3-Clause Clear License](LICENSE.txt).
