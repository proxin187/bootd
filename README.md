# bootd

bootd is a minimalist boot manager that lets you choose which profile to boot, allowing multiple boot options without the bloat of a full-featured bootloader.

## Features

- Text-based interface
- Profile-based configuration - each profile is a file on the EFI system partition
- Compatible with any kernel that can compile to an EFI boot stub, including Linux
- Only a single dependency - [uefi-rs](https://github.com/rust-osdev/uefi-rs)

## Requirements

- UEFI firmware (legacy BIOS not supported)
- Any kernel compiled as an EFI boot stub, eg. Linux compiled with `CONFIG_EFI_STUB=y`

## Installation

There is no official way to install bootd, however these are the bare minimum steps to achieve a working bootd installation on a 64-bit x86 machine:

1. Move the bootd binary into /EFI/BOOT/BOOTX64.EFI on the EFI system partition
2. Move your kernel EFI boot stub into the EFI system partition (eg. /EFI/BOOTD/kernels/vmlinuz-linux)
3. Create your [boot profiles](Profiles) (eg. /EFI/BOOTD/profiles/example_profile.bootd)
4. Ensure that the bootd binary recieves the profiles path argument `-bootd.profiles=/your/profiles/path`, on a physical machine this can be done with [efibootmgr](https://github.com/rhboot/efibootmgr)

## Profiles

A boot profile is a single file consisting of three lines:
- the profile display name line
- the kernel path line
- the kernel command line arguments line

### Example

A boot profile might look like this:
```
Linux
\EFI\BOOTD\kernels\vmlinuz-linux
root=UUID=YOUR_UUID rw loglevel=3 quiet
```

## License
bootd is licensed under the MIT license.


