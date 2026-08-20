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

const KIB_FACTOR: f64 = 1024.0;
const MEB_FACTOR: f64 = 1024.0 * 1024.0;
const GIB_FACTOR: f64 = 1024.0 * 1024.0 * 1024.0;

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
    // TODO: Improve this horrible implementation
    // TODO: Make this function auto convert from higher to lower units
    /// Auto converts the data storage unit to the most appropriate variant of the enum.
    pub fn auto_convert(self) -> Self {
        let mut output: DataStorageUnit = self;
        for _ in 0..4 {
            match self {
                DataStorageUnit::Byte(bytes) => {
                    if bytes >= 1024.0 {
                        output = output.to_kibibytes();
                        output = output.auto_convert();
                    }
                }
                DataStorageUnit::Kibibyte(kib) => {
                    if kib >= 1024.0 {
                        output = output.to_mebibytes();
                        output = output.auto_convert();
                    }
                }
                DataStorageUnit::Mebibyte(meb) => {
                    if meb >= 1024.0 {
                        output = output.to_gibibytes();
                        output = output.auto_convert();
                    }
                }
                _ => {
                    output = self;
                }
            }
        }
        output
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
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Byte(kib * KIB_FACTOR),
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Byte(meb * MEB_FACTOR),
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Byte(gib * GIB_FACTOR),
        }
    }

    pub fn to_kibibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => DataStorageUnit::Kibibyte(bytes / KIB_FACTOR),
            DataStorageUnit::Kibibyte(_) => self,
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Kibibyte(meb * KIB_FACTOR),
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Kibibyte(gib * MEB_FACTOR),
        }
    }

    pub fn to_mebibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => DataStorageUnit::Mebibyte(bytes / MEB_FACTOR),
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Mebibyte(kib / KIB_FACTOR),
            DataStorageUnit::Mebibyte(_) => self,
            DataStorageUnit::Gibibyte(gib) => DataStorageUnit::Mebibyte(gib * KIB_FACTOR),
        }
    }

    pub fn to_gibibytes(self) -> Self {
        match self {
            DataStorageUnit::Byte(bytes) => DataStorageUnit::Gibibyte(bytes / GIB_FACTOR),
            DataStorageUnit::Kibibyte(kib) => DataStorageUnit::Gibibyte(kib / MEB_FACTOR),
            DataStorageUnit::Mebibyte(meb) => DataStorageUnit::Gibibyte(meb / KIB_FACTOR),
            DataStorageUnit::Gibibyte(_) => self,
        }
    }
}

// TODO: Use a better location for the tests that use a location
// TODO: Reduce duplication
// TODO: Remove the `DataStorageUnite::` and make it so you can just do `Byte(f64)`
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

    // How to read the names of the tests below: auto_converts_to_[unit]_from_units_lower/higher_than_[unit], where [unit] is any variant of the `DataStorageUnit` enum.

    #[test]
    fn auto_converts_to_lower_kibibytes() {
        const KIB_IN_BYTES: DataStorageUnit = DataStorageUnit::Byte(1024.0);
        const KIB: DataStorageUnit = DataStorageUnit::Kibibyte(1.0);
        assert_eq!(KIB_IN_BYTES.auto_convert(), KIB);
        assert_eq!(KIB.auto_convert(), KIB);
    }

    #[test]
    fn auto_converts_to_lower_mebibytes() {
        const MEB_IN_BYTES: DataStorageUnit = DataStorageUnit::Byte(MEB_FACTOR);
        const MEB_IN_KIB: DataStorageUnit = DataStorageUnit::Kibibyte(KIB_FACTOR);
        const MEB: DataStorageUnit = DataStorageUnit::Mebibyte(1.0);

        assert_eq!(MEB_IN_BYTES.auto_convert(), MEB);
        assert_eq!(MEB_IN_KIB.auto_convert(), MEB);
        assert_eq!(MEB.auto_convert(), MEB);
    }

    #[test]
    fn auto_converts_to_lower_gibibytes() {
        const GIB_IN_BYTES: DataStorageUnit = DataStorageUnit::Byte(GIB_FACTOR);
        const GIB_IN_KIB: DataStorageUnit = DataStorageUnit::Kibibyte(MEB_FACTOR);
        const GIB_IN_MEB: DataStorageUnit = DataStorageUnit::Mebibyte(KIB_FACTOR);
        const GIB: DataStorageUnit = DataStorageUnit::Gibibyte(1.0);

        assert_eq!(GIB_IN_BYTES.auto_convert(), GIB);
        assert_eq!(GIB_IN_KIB.auto_convert(), GIB);
        assert_eq!(GIB_IN_MEB.auto_convert(), GIB);
        assert_eq!(GIB.auto_convert(), GIB);
    }

    #[test]
    fn auto_converts_to_higher_bytes() {
        const BYTES: DataStorageUnit = DataStorageUnit::Byte(1023.0);
        let bytes_in_meb = BYTES.to_mebibytes();
        let bytes_in_kib = BYTES.to_kibibytes();
        let bytes_in_gib = BYTES.to_gibibytes();

        assert_eq!(bytes_in_kib.auto_convert(), BYTES);
        assert_eq!(bytes_in_meb.auto_convert(), BYTES);
        assert_eq!(bytes_in_gib.auto_convert(), BYTES);
        assert_eq!(BYTES.auto_convert(), BYTES);
    }

    #[test]
    fn auto_converts_to_higher_kibibytes() {
        const KIB: DataStorageUnit = DataStorageUnit::Kibibyte(1023.0);
        let kib_in_meb = KIB.to_mebibytes();
        let kib_in_gib = KIB.to_gibibytes();

        assert_eq!(kib_in_meb.auto_convert(), KIB);
        assert_eq!(kib_in_gib.auto_convert(), KIB);
        assert_eq!(KIB.auto_convert(), KIB);
    }

    #[test]
    fn auto_converts_to_higher_mebibytes() {
        const MEB: DataStorageUnit = DataStorageUnit::Mebibyte(1023.0);
        let meb_in_gib = MEB.to_gibibytes();

        assert_eq!(meb_in_gib.auto_convert(), MEB);
        assert_eq!(MEB.auto_convert(), MEB);
    }
}
