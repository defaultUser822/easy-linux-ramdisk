use nix::mount::{MsFlags, mount, umount};
use nix::sys::statvfs::statvfs;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

// The documentation for the `/etc/fstab` file (The `/proc/mounts` file uses the same format): https://man7.org/linux/man-pages/man5/fstab.5.html
#[allow(unused)]
pub struct Mount {
    mount_point: String,
    filesystem_type: String, // TODO: Change this field to be an enum of all of the possible filesystems.
    mount_options: String,
}

#[allow(unused)]
pub struct MountInfo {
    total_space: u64,
    free_space: u64,
}

pub fn get_stats(filesystem: &Mount) -> io::Result<MountInfo> {
    let stats = statvfs(filesystem.mount_point.as_str())?;
    let block_size = stats.fragment_size();

    Ok(MountInfo {
        total_space: block_size * stats.blocks(),
        free_space: block_size * stats.blocks_free(),
    })
}

fn get_mounts() -> io::Result<Vec<Mount>> {
    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<Mount> = Vec::new();

    for line in lines {
        let line = line?;
        let processed_line: Vec<&str> = line.split(' ').collect();
        result.push(Mount {
            mount_options: processed_line[1].to_string(),
            filesystem_type: processed_line[2].to_string(),
            mount_point: processed_line[3].to_string(),
        });
    }
    Ok(result)
}

pub fn get_tmpfs_mounts() -> io::Result<Vec<Mount>> {
    let mounts = get_mounts()?;
    let mut result: Vec<Mount> = Vec::new();

    for mount in mounts {
        if mount.filesystem_type == "tmpfs" {
            result.push(mount);
        }
    }

    Ok(result)
}

pub fn create_ramdisk(location: &str, size: u32, uid: u32, gid: u32) -> io::Result<Mount> {
    if !fs::exists(location)? {
        fs::create_dir(location)?;
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
        mount_point: location.to_string(),
        filesystem_type: specifier.to_string(),
        mount_options: opts,
    })
}

pub fn remove_ramdisk(device: &Mount) -> io::Result<()> {
    umount(device.mount_point.as_str())?;
    Ok(())
}

// TODO: Use a better location for the tests that use a location
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_shows_tmpfs_mounts() -> io::Result<()> {
        let mounts = get_tmpfs_mounts()?;

        for mount in mounts {
            assert_eq!(mount.filesystem_type, "tmpfs");
        }

        Ok(())
    }

    #[test]
    fn mounts_correctly() -> io::Result<()> {
        let device = create_ramdisk("/home/ahmed/ramdisk2", 512, 1000, 1000)?;
        remove_ramdisk(&device)?;

        Ok(())
    }

    #[test]
    fn gets_file_stats_correctly() {
        let mebibyte: u64 = 1048576;
        let device = create_ramdisk("/home/ahmed/ramdisk", mebibyte as u32, 1000, 1000).unwrap();
        let device_stats = get_stats(&device).unwrap();
        let _ = remove_ramdisk(&device).unwrap();

        assert_eq!(device_stats.total_space, mebibyte);
    }
}
