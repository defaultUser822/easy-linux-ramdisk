# Swift disk
### Goal
A graphical program that lets the user create ramdisk devices on Linux.
### Requirements
- [ ] Core library
  - [x] Ability to list existing ramdisks that are created by the program.
    - [x] Make a function that reads mounts from /proc/mounts
  - [ ] Ability to create a ramdisk
    - [ ] Escalate privelleges somehow to allow the mounting of new filesystems.
    - [ ] Use the nix crate to mount a new tmpfs file system
  - [ ] Ability to remove a ramdisk that was created.
  - [ ] Have a default folder that the program creates under /home/USERNAME
  - [ ] Handle a variety of errors when accessing the specified folder, especially permission errors.
  - [ ] Detect existing ramdisk devices.
- [ ] Have an intuitive and useful GUI.
  - [ ] Complete decoupling of the gui from the core library.
  - [ ] Let the users choose the size of the ramdisk they want out of presets.
  - [ ] Let the users choose the folder of the ramdisk they want using a standard file dialog.
