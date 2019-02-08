# guid-partition-types-rs
A small crate designed to work with parition types and their GUIDs

## Why?
Because I was working with partition types and their GUIDs and I noticed there wasn't a crate for that. So I made one.

## Todo
 - [x] Add `System` and `PartitionType` enums
 - [x] Group GUID structs in lazy_statics
 - [ ] Document everything (In progress)

## Usage
This crate simply introduces two enums (`System` & `PartitionType`) and a struct (`GUID`)

 - The `System` enum contains variants that are named after the supported OS types, e.g `Windows, Linux, FreeBSD, Apple, ChromeOS`
 - The `PartitionType` enum contains 127 variants of all documented partition type GUIDs following the format `($SYSTEMNAME)($PARTITIONTYPE)Partition` for instance `LinuxRaidPartition` or `CephDMCryptLUKSBlockWriteAhedLogPartition`
 - The `GUID` struct simply binds a `System` and a `PartitionType` variant together with `Debug` and `Display` implemented.

 ```rust
 pub struct GUID {
    string: &'static str,
    system: Option<System>
}
```

```rust
extern crate guid_partition_types;

use guid_partition_types::{
	GUID,
	System,
	GENERAL_PARTITION_TYPES
};

fn main() {
	println!("{:?}", *GENERAL_PARTITION_TYPES.get("EFI"));
}
```
