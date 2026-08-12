//! A module for creating and removing ramdisk devices that use the tmpfs filesystem
pub mod ramdisk_mount;

/// Represents various data storage units
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataStorageUnit {
    Byte(f64),
    /// 1 Kibibyte = 1024 Bytes
    Kibibyte(f64),
    /// 1 Mebibyte = 1024 Kibibytes
    Mebibyte(f64),
    /// 1 Gebibyte = 1024 Mebibytes
    Gibibyte(f64),
}

/// Contains information about a mount
#[allow(unused)]
pub struct MountInfo {
    total_space: f64,
    free_space: f64,
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
                output.push_str(format!("{meb} MiB").as_str());
            }
            DataStorageUnit::Gibibyte(gib) => {
                output.push_str(format!("{gib} GiB").as_str());
            }
        };
        f.write_str(output.as_str())
    }
}

impl DataStorageUnit {
    pub fn auto_convert(self) -> Self {
        todo!()
    }

    fn value(&self) -> f64 {
        match self {
            DataStorageUnit::Byte(bytes) => *bytes,
            DataStorageUnit::Kibibyte(kib) => *kib,
            DataStorageUnit::Mebibyte(meb) => *meb,
            DataStorageUnit::Gibibyte(gib) => *gib,
        }
    }

    pub fn to_bytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(_) => self,
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Byte(kib * 1024.0),
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Byte(meb * 1024.0 * 1024.0),
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Byte(gib * 1024.0 * 1024.0 * 1024.0),
        }
    }

    pub fn to_kibibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => DataStorageUnit::Kibibyte(bytes / 1024.0),
            DataStorageUnit::Kibibyte(_) => self,
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Kibibyte(meb * 1024.0),
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Kibibyte(gib * 1024.0 * 1024.0),
        }
    }

    pub fn to_mebibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => DataStorageUnit::Mebibyte(bytes / (1024 * 1024) as f64),
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Mebibyte(kib / 1024.0),
            DataStorageUnit::Mebibyte(_) => self,
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Mebibyte(gib * 1024.0),
        }
    }

    pub fn to_gibibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => {
                DataStorageUnit::Gibibyte(bytes / (1024 * 1024 * 1024) as f64)
            }
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Gibibyte(kib / (1024 * 1024) as f64),
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Gibibyte(meb / 1024.0),
            DataStorageUnit::Gibibyte(_) => self,
        }
    }
}

// TODO: Use a better location for the tests that use a location
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ramdisk::ramdisk_mount::RamdiskMount;

    const GIB_IN_BYTES: DataStorageUnit = DataStorageUnit::Byte(1024.0 * 1024.0 * 1024.0);
    const GIB_IN_KIB: DataStorageUnit = DataStorageUnit::Kibibyte(1024.0 * 1024.0);
    const GIB_IN_MEB: DataStorageUnit = DataStorageUnit::Mebibyte(1024.0);
    const GIB: DataStorageUnit = DataStorageUnit::Gibibyte(1.0);

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
            DataStorageUnit::Mebibyte(1.0),
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
            DataStorageUnit::Mebibyte(1.0),
            1000,
            1000,
        )
        .unwrap();
        let device_stats = device.get_stats().unwrap();
        let _ = device.remove().unwrap();

        assert_eq!(device_stats.total_space, 1048576.0);
    }

    #[test]
    fn converts_to_bytes() {
        assert_eq!(GIB_IN_BYTES.to_bytes(), GIB_IN_BYTES);
        assert_eq!(GIB_IN_KIB.to_bytes(), GIB_IN_BYTES);
        assert_eq!(GIB_IN_MEB.to_bytes(), GIB_IN_BYTES);
        assert_eq!(GIB.to_bytes(), GIB_IN_BYTES);
    }

    #[test]
    fn converts_to_kibibytes() {
        assert_eq!(GIB_IN_BYTES.to_kibibytes(), GIB_IN_KIB);
        assert_eq!(GIB_IN_KIB.to_kibibytes(), GIB_IN_KIB);
        assert_eq!(GIB_IN_MEB.to_kibibytes(), GIB_IN_KIB);
        assert_eq!(GIB.to_kibibytes(), GIB_IN_KIB);
    }

    #[test]
    fn converts_to_mebibytes() {
        assert_eq!(GIB_IN_BYTES.to_mebibytes(), GIB_IN_MEB);
        assert_eq!(GIB_IN_KIB.to_mebibytes(), GIB_IN_MEB);
        assert_eq!(GIB_IN_MEB.to_mebibytes(), GIB_IN_MEB);
        assert_eq!(GIB.to_mebibytes(), GIB_IN_MEB);
    }

    #[test]
    fn converts_to_gibibytes() {
        assert_eq!(GIB_IN_BYTES.to_gibibytes(), GIB);
        assert_eq!(GIB_IN_KIB.to_gibibytes(), GIB);
        assert_eq!(GIB_IN_MEB.to_gibibytes(), GIB);
        assert_eq!(GIB.to_gibibytes(), GIB);
    }
}
