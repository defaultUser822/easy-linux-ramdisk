//! A module for creating and removing ramdisk devices that use the tmpfs filesystem
use nix::mount::{MsFlags, mount, umount};
use nix::sys::statvfs::statvfs;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

/// Represents various data storage units
pub enum DataStorageUnit {
    Byte(u64),
    /// 1 Kibibyte = 1024 Bytes
    Kibibyte(u64),
    /// 1 Mebibyte = 1024 Kibibytes
    Mebibyte(u64),
    /// 1 Gebibyte = 1024 Mebibytes
    Gibibyte(u64),
}

/// Contains information about a mount
#[allow(unused)]
pub struct MountInfo {
    total_space: u64,
    free_space: u64,
}

// The documentation for the `/etc/fstab` file (The `/proc/mounts` file uses the same format): https://man7.org/linux/man-pages/man5/fstab.5.html
/// Represents a mount that has the tmpfs filesystem.
#[allow(unused)]
#[derive(Clone)]
pub struct RamdiskMount {
    mount_point: String,
    filesystem_type: String,
    mount_options: String,
}

impl RamdiskMount {
    /// Creates a new `Ramdisk` mount
    ///
    /// `location` is any path on the current computer. `/home/johndoe/ramdisk`, for example.
    ///
    /// `size` is self-explanatory.
    ///
    /// `uid` is the user id of the user that owns the ramdisk mount (e.g User ID `1000`)
    ///
    /// `gid` is the group id of the group that owns the ramdisk mount (e.g Group ID `1000`)
    ///
    /// # Example
    /// ```
    /// let ramdisk_drive = RamdiskMount::new(
    ///     "/home/ahmed/ramdisk",
    ///     DataStorageUnit::Gibibyte(1),
    ///     1000,
    ///     1000,
    /// )?;
    ///
    /// let stats = device.get_stats()?;
    /// ramdisk_drive.remove()?;
    /// ```
    pub fn new(
        location: &str,
        size: DataStorageUnit,
        uid: u32,
        gid: u32,
    ) -> io::Result<RamdiskMount> {
        if !fs::exists(location)? {
            fs::create_dir(location)?;
        }
        let size = match size {
            DataStorageUnit::Byte(bytes) => format!("{bytes}"),
            DataStorageUnit::Kibibyte(kib) => format!("{kib}k"),
            DataStorageUnit::Mebibyte(meb) => format!("{meb}m"),
            DataStorageUnit::Gibibyte(gib) => format!("{gib}g"),
        };

        let opts = format!("size={size},uid={uid},gid={gid},mode=0744"); // I used an LLM for the mount options here
        let specifier = "tmpfs";
        mount(
            Some(specifier),
            location,
            Some(specifier),
            MsFlags::MS_NODEV,
            Some(opts.as_str()), // Since I didn't know about `.as_str()`, I used an LLM for this too.
        )?;
        Ok(RamdiskMount {
            mount_point: location.to_string(),
            filesystem_type: specifier.to_string(),
            mount_options: opts,
        })
    }

    /// Returns a result that may contain various statistics about the current ramdisk
    pub fn get_stats(&self) -> io::Result<MountInfo> {
        let stats = statvfs(self.mount_point.as_str())?;
        let block_size = stats.fragment_size();

        Ok(MountInfo {
            total_space: block_size * stats.blocks(),
            free_space: block_size * stats.blocks_free(),
        })
    }

    /// Unmounts the current ramdisk
    pub fn remove(&self) -> io::Result<()> {
        umount(self.mount_point.as_str())?;
        Ok(())
    }
}

impl std::fmt::Display for RamdiskMount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.mount_point)
    }
}

/// Returns a result that may all of ramdisk mounts on the current system that use the tmpfs filesystem
pub fn get_tmpfs_mounts() -> io::Result<Vec<RamdiskMount>> {
    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<RamdiskMount> = Vec::new();

    for line in lines {
        let line = line?;
        let processed_line: Vec<&str> = line.split(' ').collect();
        if processed_line[2] == "tmps" {
            result.push(RamdiskMount {
                mount_options: processed_line[1].to_string(),
                filesystem_type: processed_line[2].to_string(),
                mount_point: processed_line[3].to_string(),
            });
        }
    }

    Ok(result)
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
        let device = RamdiskMount::new(
            "/home/ahmed/ramdisk2",
            DataStorageUnit::Mebibyte(1),
            1000,
            1000,
        )?;
        device.remove()?;

        Ok(())
    }

    #[test]
    fn gets_file_stats_correctly() {
        let device = RamdiskMount::new(
            "/home/ahmed/ramdisk",
            DataStorageUnit::Mebibyte(1),
            1000,
            1000,
        )
        .unwrap();
        let device_stats = device.get_stats().unwrap();
        let _ = device.remove().unwrap();

        assert_eq!(device_stats.total_space, 1048576);
    }
}
