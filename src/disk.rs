use lazy_static::lazy_static;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat16Bpb {
    pub jmp_boot: [u8; 3],
    pub oem_name: [u8; 8],

    // BPB
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub root_entry_count: u16,
    pub total_sectors_16: u16,
    pub media: u8,
    pub fat_size_16: u16,
    pub sectors_per_track: u16,
    pub num_heads: u16,
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,

    // EBR
    pub drive_number: u8,
    pub reserved: u8,
    pub boot_signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub fs_type: [u8; 8],
}

impl Default for Fat16Bpb {
    fn default() -> Self {
        Self {
            // BPB
            jmp_boot:       [0xEB, 0x3C, 0x90],          // JMP + NOP
            oem_name:       *b"MSDOS5.0",
            bytes_per_sector: 512u16,
            sectors_per_cluster: 1,
            reserved_sectors: 1,
            num_fats: 2,
            root_entry_count: 512,
            total_sectors_16: 5856,                     // e.g. 16 MiB disk
            media: 0xF8,
            fat_size_16: 9,
            sectors_per_track: 18,
            num_heads: 2,
            hidden_sectors: 0,
            total_sectors_32: 0,

            // EBR
            drive_number: 0x80,
            reserved: 0,
            boot_signature: 0x29,
            volume_id: 0x12345678,
            volume_label: *b"BEANOS     ",
            fs_type: *b"FAT16   ",
        }
    }
}

lazy_static! {
    pub static ref BPB: Fat16Bpb = Fat16Bpb::default();
}

/// Disk Errors for disk operations
#[derive(Debug, Clone, Copy)]
pub enum DiskError {
    AMNF,
    TKZNF,
    ABRT,
    MCR,
    IDNF,
    MC,
    UNC,
    BBK
}
pub fn init_disk() -> ! {
    unimplemented!();
}

pub trait BlockDeviceSync  {
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), DiskError>;
    fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), DiskError>;
    fn sector_size(&self) -> usize;
}