# Swift disk
### Goal
A graphical program that lets the user create ramdisk devices on Linux.
### Requirements
- [x] Core library
  - [x] Ability to list existing ramdisks that are created by the program.
    - [x] Make a function that reads mounts from `/proc/mounts`
  - [x] Ability to create a ramdisk
    - [x] Use the `nix` crate to mount a new tmpfs file system
  - [x] Ability to remove a ramdisk that was created.
  - [x] Error proagation.
  - [x] Return the stats of each ramdisk using either `statvfs` or `fstatvfs` from the `nix` crate
  - [x] General improvements to the core library
- [ ] Have an intuitive and useful GUI.
  - [x] Ability to select existing ramdisk devices through a dropdown
  - [ ] Ability to remove existing ramdisk devices
  - [ ] Ability to add new ramdisk devices through a new window
