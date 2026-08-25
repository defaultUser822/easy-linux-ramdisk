use crate::ramdisk::{GIB_FACTOR, KIB_FACTOR, MEB_FACTOR};

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
