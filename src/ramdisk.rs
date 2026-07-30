use nix::mount::{MsFlags, mount, umount};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

// TODO: Remove `.unwrap`

// The documentation for the `/etc/fstab` file (The `/proc/mounts` file uses the same format): https://man7.org/linux/man-pages/man5/fstab.5.html
pub struct Mount {
    specifier: String,
    mount_point: String,
    filesystem_type: String, // TODO: Change this field to be an enum of all of the possible filesystems.
    mount_options: String,   // TODO: Change this to a `Vec<String>
    dump_frequency: String,  // TODO: Change this field to a number
    fsck_order: String,      // TODO: Change this field to a number
}

fn get_mounts() -> Vec<Mount> {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<Mount> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let processed_line: Vec<&str> = line.split(' ').collect();
        result.push(Mount {
            specifier: processed_line[0].to_string(),
            mount_options: processed_line[1].to_string(),
            filesystem_type: processed_line[2].to_string(),
            mount_point: processed_line[3].to_string(),
            dump_frequency: processed_line[4].to_string(),
            fsck_order: processed_line[5].to_string(),
        });
    }
    result
}

pub fn get_tmpfs_mounts() -> Vec<Mount> {
    let mounts = get_mounts();
    let mut result: Vec<Mount> = Vec::new();

    for mount in mounts {
        if mount.filesystem_type == "tmpfs" {
            result.push(mount);
        }
    }

    result
}

pub fn create_ramdisk(size: u32, uid: u32, gid: u32) -> nix::Result<Mount> {
    let location = "/home/ahmed/ramdisk";
    if !fs::exists(location).unwrap() {
        fs::create_dir("/home/ahmed/ramdisk").unwrap();
    }
    let mode = 0744;

    let opts = format!("size={size},uid={uid},gid={gid},mode={mode}"); // I used an LLM for the mount options here
    let specifier = "tmpfs";
    mount(
        Some(specifier),
        location,
        Some(specifier),
        MsFlags::MS_NODEV,
        Some(opts.as_str()), // Since I didn't know about `.as_str()`, I used an LLM for this too.
    )?;
    Ok(Mount {
        specifier: specifier.to_string(),
        mount_point: location.to_string(),
        filesystem_type: specifier.to_string(),
        mount_options: opts,
        dump_frequency: "0".to_string(),
        fsck_order: "0".to_string(),
    })
}

pub fn remove_ramdisk(device: Mount) -> nix::Result<()> {
    umount(device.mount_point.as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_shows_tmpfs_mounts() {
        let mounts = get_tmpfs_mounts();

        for mount in mounts {
            assert_eq!(mount.filesystem_type, "tmpfs");
        }
    }

    #[test]
    fn mounts_correctly() -> nix::Result<()> {
        let device = create_ramdisk(512, 1000, 1000)?;
        remove_ramdisk(device)?;

        Ok(())
    }
}
