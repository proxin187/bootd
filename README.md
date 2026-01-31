# bootd

bootd is a minimal, composable EFI launcher. It displays a list of profiles and executes the selected EFI binary.

## What is bootd?

bootd is not a traditional boot manager - it's an **EFI launcher**, imagine dmenu for UEFI.

Because profiles can point to any EFI binary (including bootd itself), you can:
- Boot multiple operating systems
- Create nested menu systems
- Launch system utilities (memory tests, diagnostics)
- Restart into the firmware setup
- Execute custom EFI applications

## Features

- **Minimal**: ~60KB binary, single dependency ([uefi-rs](https://github.com/rust-osdev/uefi-rs))
- **Composable**: Features through composition, not complexity
- **Intentional**: Every action is intentional - no defaults
- **Fast**: Text-based interface - real accessibility, no complex graphical interfaces

## Profiles

A profile is a three-line file that describes an EFI executable:
1. Display name
2. EFI executable path
3. Command line arguments

### Example: Booting Linux
```
Arch Linux
\EFI\BOOTD\kernels\vmlinuz-linux
root=UUID=YOUR_UUID rw loglevel=3 quiet
```

### Example: Nested Menu
```
System Utilities >
\EFI\BOOT\BOOTX64.EFI
--profiles \EFI\BOOTD\profiles.utilities
```

## Requirements

- UEFI firmware (legacy BIOS not supported)
- EFI binaries to launch (eg. A Linux kernel compiled with `CONFIG_EFI_STUB=y`)

## Installation

There is no official way to install bootd, however here are the bare minimum steps for a working bootd installation on x86_64:

1. Move the bootd binary into /EFI/BOOT/BOOTX64.EFI on the EFI system partition
2. Move your EFI binary to the EFI system partition (eg. /EFI/BOOTD/kernels/vmlinuz-linux)
3. Create your profiles (eg. /EFI/BOOTD/profiles/arch-linux)
4. Configure bootd to read the correct profiles path by passing the profiles directory as an argument `-profiles /your/profiles/path`, on a physical machine this can be done with efibootmgr

### Directory Structure

An installation can look like this:
```
/EFI/
├── BOOT/
│   └── BOOTX64.EFI
├── BOOTD/
│   ├── profiles/
│   │   ├── arch-linux
│   │   ├── arch-linux-lts
│   │   └── utilities
│   ├── profiles.utilities/
│   │   ├── firmware
│   │   └── back
│   ├── kernels/
│   │   ├── vmlinuz-linux
│   │   └── vmlinuz-linux-lts
│   └── addons/
│       └── system-reset.efi
```

## Addons

The `addons/` directory contains optional EFI tools you can use in your profiles:

- system-reset - A tool that wraps around [ResetSystem](https://uefi.org/specs/UEFI/2.10/08_Services_Runtime_Services.html#resetsystem) (cold/warm reboot, shutdown, firmware setup)

## License
bootd is licensed under the MIT license.


