# Swift disk
### Goal
A graphical program that lets the user create ramdisk devices on Linux.
### Requirements
- [x] Core library
  - [x] Ability to list existing ramdisks that are created by the program.
    - [x] Make a function that reads mounts from /proc/mounts
  - [x] Ability to create a ramdisk
    - [x] Use the nix crate to mount a new tmpfs file system
  - [x] Ability to remove a ramdisk that was created.
  - [x] Error proagation.
- [ ] Have an intuitive and useful GUI.
  - [ ] Complete decoupling of the gui from the core library.
  - [ ] Let the users choose the size of the ramdisk they want out of presets.
  - [ ] Let the users choose the folder of the ramdisk they want using a standard file dialog.
