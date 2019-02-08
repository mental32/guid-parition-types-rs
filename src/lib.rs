// #![deny(missing_docs)]
#![feature(proc_macro_hygiene)]

extern crate lazy_static;

use lazy_static::lazy_static;

use std::fmt;
use std::collections::HashMap;

pub mod types;
pub use self::types::{System, PartitionType};

pub struct GUID {
    string: &'static str,
    system: Option<System>
}

impl fmt::Debug for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GUID{{ {:?} => {:?} }}", self.system, self.string)
    }
}

impl fmt::Display for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

macro_rules! guid {
    ($string:expr, $system:expr) => (GUID { string: $string, system: $system})
}

macro_rules! fillmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

lazy_static!{
    pub static ref GENERAL_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "unused"         => guid!("00000000-0000-0000-0000-000000000000", None),
            "MBR"            => guid!("024DEE41-33E7-11D3-9D69-0008C781F39F", None),
            "EFI"            => guid!("C12A7328-F81F-11D2-BA4B-00A0C93EC93B", None),
            "BIOSBoot"       => guid!("21686148-6449-6E6F-744E-656564454649", None),
            "IntelFastFlash" => guid!("D3BFE2DE-3DAF-11DF-BA40-E3A556D89593", None),
            "SonyBoot"       => guid!("F4019732-066E-4E12-8273-346C5641494F", None),
            "LenovoBoot"     => guid!("BFBFAFE7-A34F-448A-9A5B-6213EB736C22", None)
        ]
    };

    pub static ref WINDOWS_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "MSR"           => guid!("E3C9E316-0B5C-4DB8-817D-F92DF00215AE", Some(System::Windows)),
            "BasicData"     => guid!("EBD0A0A2-B9E5-4433-87C0-68B6B72699C7", Some(System::Windows)),
            "LDMMetadata"   => guid!("5808C8AA-7E8F-42E0-85D2-E1E90434CFB3", Some(System::Windows)),
            "LDMData"       => guid!("AF9B60A0-1431-4F62-BC68-3311714A69AD", Some(System::Windows)),
            "RE"            => guid!("DE94BBA4-06D1-4D40-A16A-BFD50179D6AC", Some(System::Windows)),
            "GPFS"          => guid!("37AFFC90-EF7D-4E96-91C3-2D7AE055B174", Some(System::Windows)),
            "StorageSpaces" => guid!("E75CAF8F-F680-4CEE-AFA3-B001E56EFC2D", Some(System::Windows)),
            "HPUXData"      => guid!("75894C1E-3AEB-11D3-B7C1-7B03A0000000", Some(System::Windows)),
            "Service"       => guid!("E2A1E728-32E3-11D6-A682-7B03A0000000", Some(System::Windows))
        ]
    };

    pub static ref LINUX_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "filesystem data"      => guid!("0FC63DAF-8483-4772-8E79-3D69D8477DE", Some(System::Linux)),
            "RAID"                 => guid!("A19D880F-05FC-4D3B-A006-743F0F84911", Some(System::Linux)),
            "Rootx86"              => guid!("44479540-F297-41B2-9AF7-D131D5F0458", Some(System::Linux)),
            "Rootx86-64"           => guid!("4F68BCE3-E8CD-4DB1-96E7-FBCAF984B70", Some(System::Linux)),
            "Root32-bit ARM"       => guid!("69DAD710-2CE4-4E3C-B16C-21A1D49ABED", Some(System::Linux)),
            "Root64-bit ARM"       => guid!("B921B045-1DF0-41C3-AF44-4C6F280D3FA", Some(System::Linux)),
            "Swap"                 => guid!("0657FD6D-A4AB-43C4-84E5-0933C84B4F4", Some(System::Linux)),
            "LogicalVolumeManager" => guid!("E6D6D379-F507-44C2-A23C-238F2A3DF92", Some(System::Linux)),
            "home"                 => guid!("933AC7E1-2EB4-4F13-B844-0E14E2AEF91", Some(System::Linux)),
            "srv"                  => guid!("3B8F8425-20E0-4F3B-907F-1A25A76F98E", Some(System::Linux)),
            "dm-crypt"             => guid!("7FFEC5C9-2D00-49B7-8941-3EA10A5586B", Some(System::Linux)),
            "LUKS"                 => guid!("CA7D7CCB-63ED-4C53-861C-1742536059C", Some(System::Linux)),
            "Reserved"             => guid!("8DA63339-0007-60C0-C436-083AC823090", Some(System::Linux))
        ]
    };

    pub static ref FREEBSD_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "FreeBSD-Boot"               => guid!("83BD6B9D-7F41-11DC-BE0B-001560B84F0F", Some(System::FreeBSD)),
            "FreeBSD-Data"               => guid!("516E7CB4-6ECF-11D6-8FF8-00022D09712B", Some(System::FreeBSD)),
            "FreeBSD-Swap"               => guid!("516E7CB5-6ECF-11D6-8FF8-00022D09712B", Some(System::FreeBSD)),
            "FreeBSD-UnixFileSystem"     => guid!("516E7CB6-6ECF-11D6-8FF8-00022D09712B", Some(System::FreeBSD)),
            "FreeBSD-VinumVolumeManager" => guid!("516E7CB8-6ECF-11D6-8FF8-00022D09712B", Some(System::FreeBSD)),
            "FreeBSD-ZFS"                => guid!("516E7CBA-6ECF-11D6-8FF8-00022D09712B", Some(System::FreeBSD))
        ]
    };

    pub static ref APPLE_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "HFS+"                => guid!("48465300-0000-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "APFS"                => guid!("7C3457EF-0000-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "UFS container"       => guid!("55465300-0000-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "ZFS"                 => guid!("6A898CC3-1DD2-11B2-99A6-080020736631", Some(System::Apple)),
            "RAID"                => guid!("52414944-0000-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "RAID offline"        => guid!("52414944-5F4F-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "Boot"                => guid!("426F6F74-0000-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "Label"               => guid!("4C616265-6C00-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "TV Recovery"         => guid!("5265636F-7665-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "Core Storage"        => guid!("53746F72-6167-11AA-AA11-00306543ECAC", Some(System::Apple)),
            "SoftRAID_Status"     => guid!("B6FA30DA-92D2-4A9A-96F1-871EC6486200", Some(System::Apple)),
            "SoftRAID_Scratch"    => guid!("2E313465-19B9-463F-8126-8A7993773801", Some(System::Apple)),
            "SoftRAID_Volume"     => guid!("FA709C7E-65B1-4593-BFD5-E71D61DE9B02", Some(System::Apple)),
            "SoftRAID_Cache"      => guid!("BBBA6DF5-F46F-4A89-8F59-8765B2727503", Some(System::Apple))
        ]
    };

    pub static ref SOLARIS_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Boot"             => guid!("6A82CB45-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Root"             => guid!("6A85CF4D-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Swap"             => guid!("6A87C46F-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Backup"           => guid!("6A8B642B-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "/usr"             => guid!("6A898CC3-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "/var"             => guid!("6A8EF2E9-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "/home"            => guid!("6A90BA39-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Alternate sector" => guid!("6A9283A5-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Reserved"         => guid!("6A945A3B-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Reserved1"        => guid!("6A9630D1-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Reserved2"        => guid!("6A980767-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Reserved3"        => guid!("6A96237F-1DD2-11B2-99A6-080020736631", Some(System::Solaris)),
            "Reserved4"        => guid!("6A8D2AC7-1DD2-11B2-99A6-080020736631", Some(System::Solaris))
        ]
    };

    pub static ref NETBSD_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Swap"          => guid!("49F48D32-B10E-11DC-B99B-0019D1879648", Some(System::NetBSD)),
            "FFS"           => guid!("49F48D5A-B10E-11DC-B99B-0019D1879648", Some(System::NetBSD)),
            "LFS"           => guid!("49F48D82-B10E-11DC-B99B-0019D1879648", Some(System::NetBSD)),
            "RAID"          => guid!("49F48DAA-B10E-11DC-B99B-0019D1879648", Some(System::NetBSD)),
            "Concatenated"  => guid!("2DB519C4-B10F-11DC-B99B-0019D1879648", Some(System::NetBSD)),
            "Encrypted"     => guid!("2DB519EC-B10F-11DC-B99B-0019D1879648", Some(System::NetBSD))
        ]
    };

    pub static ref CHROMEOS_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "kernel" => guid!("FE3A2A5D-4F32-41A7-B725-ACCC3285A309", Some(System::ChromeOS)),
            "rootfs" => guid!("3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC", Some(System::ChromeOS)),
            "future" => guid!("2E0A753D-9E48-43B0-8337-B15192CB1B5E", Some(System::ChromeOS))
        ]
    };

    pub static ref COREOS_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Usr)"      => guid!("5DFBF5F4-2848-4BAC-AA5E-0D9A20B745A6", Some(System::CoreOS)),
            "Resize)"   => guid!("3884DD41-8582-4404-B9A8-E9B84F2DF50E", Some(System::CoreOS)),
            "Reserved)" => guid!("C95DC21A-DF0E-4340-8D7B-26CBFA9A03E0", Some(System::CoreOS)),
            "RootRaid)" => guid!("BE9067B9-EA49-4F15-B4F6-F36F8C9E1818", Some(System::CoreOS))
        ]
    };

    pub static ref HAIKU_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "BFS" => guid!("42465331-3BA3-10F1-802A-4861696B7521", Some(System::Haiku))
        ]
    };

    pub static ref MIDNIGHTBSD_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Boot"                    => guid!("85D5E45E-237C-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD)),
            "Data"                    => guid!("85D5E45A-237C-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD)),
            "Swap"                    => guid!("85D5E45B-237C-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD)),
            "Unix File System (UFS)"  => guid!("0394EF8B-237E-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD)),
            "Vinum volume manager"    => guid!("85D5E45C-237C-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD)),
            "ZFS"                     => guid!("85D5E45D-237C-11E1-B4B3-E89A8F7FC3A7", Some(System::MidnightBSD))
        ]
    };

    pub static ref CEPH_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Journal"                             => guid!("45B0969E-9B03-4F30-B4C6-B4B80CEFF106", Some(System::Ceph)),
            "dm-crypt journal"                    => guid!("45B0969E-9B03-4F30-B4C6-5EC00CEFF106", Some(System::Ceph)),
            "OSD"                                 => guid!("4FBD7E29-9D25-41B8-AFD0-062C0CEFF05D", Some(System::Ceph)),
            "dm-crypt OSD"                        => guid!("4FBD7E29-9D25-41B8-AFD0-5EC00CEFF05D", Some(System::Ceph)),
            "Disk in creation"                    => guid!("89C57F98-2FE5-4DC0-89C1-F3AD0CEFF2BE", Some(System::Ceph)),
            "dm-crypt disk in creation"           => guid!("89C57F98-2FE5-4DC0-89C1-5EC00CEFF2BE", Some(System::Ceph)),
            "Block"                               => guid!("CAFECAFE-9B03-4F30-B4C6-B4B80CEFF106", Some(System::Ceph)),
            "Block DB"                            => guid!("30CD0809-C2B2-499C-8879-2D6B78529876", Some(System::Ceph)),
            "Block write-ahead log"               => guid!("5CE17FCE-4087-4169-B7FF-056CC58473F9", Some(System::Ceph)),
            "Lockbox for dm-crypt keys"           => guid!("FB3AABF9-D25F-47CC-BF5E-721D1816496B", Some(System::Ceph)),
            "Multipath OSD"                       => guid!("4FBD7E29-8AE0-4982-BF9D-5A8D867AF560", Some(System::Ceph)),
            "Multipath journal"                   => guid!("45B0969E-8AE0-4982-BF9D-5A8D867AF560", Some(System::Ceph)),
            "Multipath block"                     => guid!("CAFECAFE-8AE0-4982-BF9D-5A8D867AF560", Some(System::Ceph)),
            "Multipath block"                     => guid!("7F4A666A-16F3-47A2-8445-152EF4D03F6C", Some(System::Ceph)),
            "Multipath block DB"                  => guid!("EC6D6385-E346-45DC-BE91-DA2A7C8B3261", Some(System::Ceph)),
            "Multipath block write-ahead log"     => guid!("01B41E1B-002A-453C-9F17-88793989FF8F", Some(System::Ceph)),
            "dm-crypt block"                      => guid!("CAFECAFE-9B03-4F30-B4C6-5EC00CEFF106", Some(System::Ceph)),
            "dm-crypt block DB"                   => guid!("93B0052D-02D9-4D8A-A43B-33A3EE4DFBC3", Some(System::Ceph)),
            "dm-crypt block write-ahead log"      => guid!("306E8683-4FE2-4330-B7C0-00A917C16966", Some(System::Ceph)),
            "dm-crypt LUKS journal"               => guid!("45B0969E-9B03-4F30-B4C6-35865CEFF106", Some(System::Ceph)),
            "dm-crypt LUKS block"                 => guid!("CAFECAFE-9B03-4F30-B4C6-35865CEFF106", Some(System::Ceph)),
            "dm-crypt LUKS block DB"              => guid!("166418DA-C469-4022-ADF4-B30AFD37F176", Some(System::Ceph)),
            "dm-crypt LUKS block write-ahead log" => guid!("86A32090-3647-40B9-BBBD-38D8C573AA86", Some(System::Ceph)),
            "dm-crypt LUKS OSD"                   => guid!("4FBD7E29-9D25-41B8-AFD0-35865CEFF05D", Some(System::Ceph))
        ]
    };

    pub static ref OPENBSD_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Data"  => guid!("824CC7A0-36A8-11E3-890A-952519AD3F61", Some(System::OpenBSD))
        ]
    };

    pub static ref QNX_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "PowerSafe" => guid!("CEF5A9AD-73BC-4601-89F3-CDEEEEE321A1", Some(System::QNX))
        ]
    };

    pub static ref VMWARE_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "vmkcore"  => guid!("9D275380-40AD-11DB-BF97-000C2911D1B8", Some(System::VMware)),
            "VMFS"     => guid!("AA31E02A-400F-11DB-9590-000C2911D1B8", Some(System::VMware)),
            "Reserved" => guid!("9198EFFC-31C0-11DB-8F78-000C2911D1B8", Some(System::VMware))
        ]
    };

    pub static ref ANDROID_IA_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Bootloader"          => guid!("2568845D-2332-4675-BC39-8FA5A4748D15", Some(System::AndroidIA)),
            "Bootloader2"         => guid!("114EAFFE-1552-4022-B26E-9B053604CF84", Some(System::AndroidIA)),
            "Boot"                => guid!("49A4D17F-93A3-45C1-A0DE-F50B2EBE2599", Some(System::AndroidIA)),
            "Recovery"            => guid!("4177C722-9E92-4AAB-8644-43502BFD5506", Some(System::AndroidIA)),
            "Misc"                => guid!("EF32A33B-A409-486C-9141-9FFB711F6266", Some(System::AndroidIA)),
            "Metadata"            => guid!("20AC26BE-20B7-11E3-84C5-6CFDB94711E9", Some(System::AndroidIA)),
            "System"              => guid!("38F428E6-D326-425D-9140-6E0EA133647C", Some(System::AndroidIA)),
            "Cache"               => guid!("A893EF21-E428-470A-9E55-0668FD91A2D9", Some(System::AndroidIA)),
            "Data"                => guid!("DC76DDA9-5AC1-491C-AF42-A82591580C0D", Some(System::AndroidIA)),
            "Persistent"          => guid!("EBC597D0-2053-4B15-8B64-E0AAC75F4DB1", Some(System::AndroidIA)),
            "Vendor"              => guid!("C5A0AEEC-13EA-11E5-A1B1-001E67CA0C3C", Some(System::AndroidIA)),
            "Config"              => guid!("BD59408B-4514-490D-BF12-9878D963F378", Some(System::AndroidIA)),
            "Factory"             => guid!("8F68CC74-C5E5-48DA-BE91-A0C8C15E9C80", Some(System::AndroidIA)),
            "Factory (alt)"       => guid!("9FDAA6EF-4B3F-40D2-BA8D-BFF16BFB887B", Some(System::AndroidIA)),
            "Fastboot / Tertiary" => guid!("767941D0-2085-11E3-AD3B-6CFDB94711E9", Some(System::AndroidIA)),
            "OEM"                 => guid!("AC6D7924-EB71-4DF8-B48D-E267B27148FF", Some(System::AndroidIA))
        ]
    };

    pub static ref ANDROID_ARM_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Android Meta" => guid!("19A710A2-B3CA-11E4-B026-10604B889DCF", Some(System::AndroidARM)),
            "Android EXT"  => guid!("193D1EA4-B3CA-11E4-B075-10604B889DCF", Some(System::AndroidARM))
        ]
    };

    pub static ref ONIE_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Boot"   => guid!("7412F7D5-A156-4B13-81DC-867174929325", Some(System::ONIE)),
            "Config" => guid!("D4E6E2CD-4469-46F3-B5CB-1BFF57AFC149", Some(System::ONIE))
        ]
    };

    pub static ref ATARITOS_PARTITION_TYPES: HashMap<&'static str, GUID> = {
        fillmap![
            "Basic data" => guid!("734E5AFE-F61A-11E6-BC64-92361F002671", Some(System::AtariTOS))
        ]
    };
}
