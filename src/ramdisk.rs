//! A module for creating and removing ramdisk devices that use the tmpfs filesystem
pub mod ramdisk_mount;

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

impl MountInfo {
    /// Returns the total amount space of the mount in bytes
    pub fn total_space(&self) -> DataStorageUnit {
        DataStorageUnit::Byte(self.total_space)
    }

    /// Returns the amount of free space of the mount in bytes
    pub fn free_space(&self) -> DataStorageUnit {
        DataStorageUnit::Byte(self.free_space)
    }
}

impl std::fmt::Display for DataStorageUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_string();
        match self {
            DataStorageUnit::Byte(bytes) => {
                output.push_str(format!("{bytes} B").as_str());
            }
            DataStorageUnit::Kibibyte(kib) => {
                output.push_str(format!("{kib} KiB").as_str());
            }
            DataStorageUnit::Mebibyte(meb) => {
                output.push_str(format!("{meb} MeB").as_str());
            }
            DataStorageUnit::Gibibyte(gib) => {
                output.push_str(format!("{gib} GiB").as_str());
            }
        };
        f.write_str(output.as_str())
    }
}

// TODO: Use a better location for the tests that use a location
#[cfg(test)]
mod tests {
    use crate::ramdisk::ramdisk_mount::RamdiskMount;

    use super::*;

    #[test]
    fn only_shows_tmpfs_mounts() -> std::io::Result<()> {
        let mounts = RamdiskMount::from_existing(false)?;

        for mount in mounts {
            assert_eq!(mount.filesystem_type(), "tmpfs");
        }

        Ok(())
    }

    #[test]
    fn mounts_correctly() -> std::io::Result<()> {
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
