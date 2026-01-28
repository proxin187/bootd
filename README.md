# bootd

bootd is a minimalist boot manager that does one thing and does it well: let you choose which profile to boot.

## Features
- text-based interface
- profile-based configuration - each profile is a file on the EFI system partition
- compatible with any OS that can compile as an EFI boot stub, including linux

## Why use a boot manager instead of bootloader?
A bootloader handles the entire boot process. A boot manager only handles selection of what to boot and lets the [EFI boot stub](https://docs.kernel.org/admin-guide/efi-stub.html) handle the rest.

For most systems, a boot manager is more than enough. You don't need recovery shells, fancy UI, support for 20 different file systems or scripting languages just to select which kernel to boot.

## Requirements

- UEFI firmware (legacy BIOS not supported for obvious reasons)
- Any kernel compiled as an EFI boot stub, eg. Linux compiled with `CONFIG_EFI_STUB=y`

## License
bootd is licensed under the MIT license.


