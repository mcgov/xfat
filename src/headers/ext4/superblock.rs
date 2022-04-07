use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Superblock {
    pub inodes: u32,                  //Total number of inodes in file system
    pub blocks: u32,                  //Total number of blocks in file system
    pub reserved_blocks: u32,         //Number of reserved blocks
    pub blocks_left: u32,             //Total number of unallocated blocks
    pub inodes_left: u32,             //Total number of unallocated inodes
    pub superblock: u32, //Block number of the block containing the superblock. This is 1 on 1024 byte block size filesystems, and 0 for all others.
    pub block_shift: u32, //log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    pub fragment_shift: u32, //log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    pub group_blocks: u32,   //Number of blocks in each block group
    pub group_fragments: u32, //Number of fragments in each block group
    pub group_inodes: u32,   //Number of inodes in each block group
    pub last_mount_at: u32,  //Last mount time (in POSIX time)
    pub last_written_at: u32, //Last written time (in POSIX time)
    pub mounts_since_last_check: u16, //Number of times the volume has been mounted since its last consistency check (fsck)
    pub max_mounts_before_check: u16, //Number of mounts allowed before a consistency check (fsck) must be done
    pub magic: u16, //Magic signature (0xef53), used to help confirm the presence of Ext4 on a volume
    pub fs_state: u16, //File system state.
    pub error_action: u16, //What to do when an error is detected
    pub version_minor: u16, //Minor portion of version (combine with Major portion below to construct full version field)
    pub last_check_at: u32, //POSIX time of last consistency check (fsck)
    pub forced_check_interval: u32, //Interval (in POSIX time) between forced consistency checks (fsck)
    pub os_id: u32, //Operating system ID from which the filesystem on this volume was created (see below)
    pub version_major: u32, //Major portion of version (combine with Minor portion above to construct full version field)
    pub uid_owner: u16,     //User ID that can use reserved blocks
    pub gid_owner: u16,     //Group ID that can use reserved blocks
}
