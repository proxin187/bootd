# bootd

bootd is a minimalist boot manager for linux, bootd displays boot options and boots a [EFI boot stub](https://docs.kernel.org/admin-guide/efi-stub.html) with the respective command-line parameters.

## Features
- Simple text-based interface
- Able to boot linux

## Why use a boot manager instead of bootloader?
A bootloader handles the entire boot process. A boot manager only handles selection of what to boot and lets the [EFI boot stub](https://docs.kernel.org/admin-guide/efi-stub.html) handle the rest.

For most systems, a boot manager is more than enough. You don't need recovery shells, fancy UI, support for 20 different file systems or scripting languages just to select which kernel to boot.

## License
bootd is licensed under the MIT license.


