# Swift disk
### Goal
A graphical program that lets the user create ramdisk devices on Linux.
### Requirements
- [ ] Core library
  - [x] Ability to list existing ramdisks that are created by the program.
    - [x] Make a function that reads mounts from `/proc/mounts`
  - [x] Ability to create a ramdisk
    - [x] Use the `nix` crate to mount a new tmpfs file system
  - [x] Ability to remove a ramdisk that was created.
  - [x] Error proagation.
  - [ ] Return the stats of each ramdisk using either `statvfs` or `fstatvfs` from the `nix` crate
- [ ] Have an intuitive and useful GUI.
  - [ ] TODO
