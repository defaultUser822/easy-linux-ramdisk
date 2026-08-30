# Easy Linux Ramdisk

### Table of contents
- [Overiew](#overview)
- [Why](#why)
- [What I learned](#what-i-learned)
- [How I did it](#how-i-did-it)
  - [Program organization & general approach](#program-organization--general-approach)
  - [Getting the existing mounted ramdisk files](#getting-the-existing-mounted-ramdisk-file)
  - [Getting the statistics of each mounted ramdisk](#getting-the-statistics-of-each-mounted-ramdisk)
- [How to use this tool](#how-to-use-this-tool)
- [Requirements](#requirements)
- [Tools used](#tools-used)
- [Technical decisions](#technical-decisions)
- [AI and external code snippet usage disclosure](#ai-and-external-code-snippet-usage-disclosure)
  

### Overview
A graphical tool to create, delete, and explore ramdisk devices that are made with the `tmpfs` file system. This program will work without extra dependencies on most modern Linux distributions (e.g, Ubuntu, Fedora, Bazzite, Arch).

### Why
Graphical tools are just convienient beccaus they eliminate the need to memorize command or anything. There was absolutely nothing like this tool as far as I can tell. It was a unique project idea that I personally liked.

### What I learned
I learned a lot about the Linux file permission system and Linux itself in general. Linux has a bigger set of functions than I had previously thought. 

I also learned the importance of defining a thorough and clear requirments list before starting the project. Throughout this project's development, I kept adding requirements because I wasn't specific enough when I first wrote it. You can check the [requirements](#requirements) section for an insight on the final requirements list. This list, despite its length, is still somewhat vague on the UI section. The importance of the existance of a requirements list appeared when I tried to make another project but I just kept adding random stuff and never felt that it was "done". 

I also learned, from my first attempted project as well as expriences in the distant past, which I made sure to never repeat, that worrying too much about the structure of your project early on can only make things hard. You can always refactor everything later!

### How I did it
#### Program organization & general approach
I divided the app into two parts: the core library and the GUI. For the core library, instead of just using a `mount` command with some option, I decided to use a more programmatic approach. I used the `nix` crate (similar to a library), which provides "safe"/Rust friendly wrappers over libc functions on *nix systems. While this approach requires more effore, it gives better results. When, for example, mounting a new ramdisk device fails, the error can be returned instead of being thrown into the void. Most of the core library's functionality is under the `RamdiskMount` struct. 

#### Getting the existing mounted ramdisk files
For getting the existing mount stats, I just read the `/proc/mounts` file, which uses the same format as the `/etc/fstab` file, and filter the ones that aren't of the `tmpfs` filesystem. There is also an option to the user of the core library to show or hide system mounts of the `tmpfs` file system. You can check the method `.from_existing()` that's associated with the `RamdiskMount` struct for more details on how I implement this functionality.

#### Getting the statistics of each mounted ramdisk
I used the `statvfs()` function from the `nix` crate. I multiplied the fragment size, which is how big each block of the mount is in bytes, by the number of blocks to get the total space and then by the number of the number of free storage to get the free space.

### How to use this tool
1. Download the binary from the releases
2. Make the file executable
3. Fix the permissions using this command ```sudo setcap 'cap_sys_admin=ep' ./easy-linux-ramdisk```
4. Click "Create New Ramdisk"
5. Choose the folder where you want the ramdisk to be created
6. Choose the desired size of the ramdisk
7. Click "Create Ramdisk"
8. Done 🎉

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
- [x] Have an intuitive and useful GUI.
  - [x] Ability to select existing ramdisk devices through a dropdown
  - [x] Ability to remove existing ramdisk devices
  - [x] Ability to add new ramdisk devices through a new window or page
  - [x] Proper error propagation to the user for when something goes wrong
  - [x] Ability to choose custom folders
  
### Tools used
**Programming language:** Rust

**GUI library:** Iced

**Version Control System:** Git

**LLM:** Google Gemini

**Build system/Command runner**: Cargo and Just

### Technical decisions
I decided to use Rust because it is the only language that I know besides Python and C. Python would have been unsuitable for a project that directly uses libc like this. I don't think C would have been particularly enjoyable to use in a project involving a GUI because of how low level it is.

Deciding to use Rust came with some hurdles, especially that I want to make a GUI. Most of the GUI frameworks available in the Rust ecosystem are immature. The main exception is Slint and Qt with Qt bridges when it is out of its beta stage. I considered using Slint in the beginning but I skipped on it because I was unfamiliar with the DSL it used. So I went with Iced. It is also an immature option but it is the one I am most familiar with.

I also decided to use Just for simplifing the commands that are common during development and for making cargo, which is Rust's package manager, use sccache. For example, instead of writing out the command `sudo setcap 'cap_sys_admin=ep' [binary_file]` everytime I needed to fix the permissions, which is needed so the program can mount ramdisks, I just type out `just fix-perms [binary_file]`.
### AI and external code snippet usage disclosure
I used some code for setting up the GUI framework from a previous project I had used. It's just boilerplate that can't be explicity outlined. I was just a little lazy. 
I also read some examples from the GUI framework when I was stuck. No code was directly used.
I also used 2 lines of code from an LLM, which are explicitly outlined in the comments.
I also used it for researching ideas and explaining conecpts of the Linux kernel.
