use vm_memory::ByteValued;

/// Config as defined in https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/virtio_blk.h#L60
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct Config {
    /// Capacity expressed in sectors, 1 sector is 512 bytes (1<<9).
    capacity: u64,
    /// Maximum segment size, only if VIRTIO_BLK_F_SIZE_MAX is negotiated.
    size_max: u32,
    /// Maximum number of segments, if VIRTIO_BLK_F_SEG_MAX is negotiated.
    seg_max: u32,
    /// Geometry for this device.
    geometry: Geometry,
    /// Size of a single block on the deivce, if VIRTIO_BLK_F_SIZE is negotiated.
    blk_size: u32,
    /// Exponent for physical block per logical block, i.e. every logical block is
    /// 2^physical_block_exp physical blocks, if VIRTIO_BLK_F_TOPOLOGY is negotiated.
    physical_block_exp: u8,
    /// Alignment offset expressed in logical blocks, if VIRTIO_BLK_F_TOPOLOGY is negotiated.
    alignment_offset: u8,
    /// Minimum I/O size without performance penalty in logical blocks, if
    /// VIRTIO_BLK_F_TOPOLOGY is negotiated.
    min_io_size: u16,
    /// Optimal sustained I/O size in logcial blocks, if VIRTIO_BLK_F_TOPOLOGY is negotiated.
    opt_io_size: u16,
    /// Writeback mode, if VIRTIO_BLK_F_CONFIG_WCE is negotiated.
    wce: u8,
    /// Unused space.
    unused: u8,
    /// Amount of vritqueues, if VIRTIO_BLK_F_MQ is negotiated.
    num_queues: u16,
    /// Maximum discard sectors (in 512-byte sectors) for one segment, if VIRTIO_BLK_F_DISCARD is
    /// negotiated.
    max_discared_sectors: u32,
    /// The maximum nuber of discard segments in a discard command, if VIRTIO_BLK_F_DISCARD is
    /// negotiated.
    max_discard_seq: u32,
    /// Discard commands must be aligned ot this number of sectors, if VIRTIO_BLK_F_DISCARD is
    /// negotiated.
    discard_sector_alignment: u32,
    /// The maximum number of write zeroes sectors (in 512-byte sectors) in one segment, if
    /// VIRTIO_BLK_F_WRITE_ZEROES is negotiated.
    max_write_zeroes_sectors: u32,
    /// The maximum number of segments in a write zeroes command, if VIRTIO_BLK_F_WRITE_ZEROES is
    /// negotiated.
    max_write_zeroes_seg: u32,
    /// Set is a VIRTIO_BLK_T_WRITE_ZEROES request may result in the deallocation of one or more
    /// sectors.
    write_zeroes_may_unmap: u8,
    /// Unused space.
    unused1: [u8; 3],
    /// The maximum secure erase sectors (in 512-byte sectors) for one segment, if
    /// VIRTIO_BLK_F_SECURE_ERASE is negotiated.
    max_secure_erase_sectors: u32,
    /// The maximum number of secure erase segments in a secure erase command, if
    /// VIRTIO_BLK_F_SECURE_ERASE is negotiated.
    max_secure_erase_seg: u32,
    /// Secure erase commands must be aligned to this number of sectors, if
    /// VIRTIO_BLK_F_SECURE_ERASE is negotiated.
    secure_erase_sector_alignment: u32,
    /// Zoned block device characteristics, if VIRTIO_BLK_F_ZONED is negotiated.
    zoned: ZoneCharacteristic,
}

/// Geometry of a device.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct Geometry {
    cylinders: u16,
    heads: u8,
    sectors: u8,
}

/// Zone information of a device.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct ZoneCharacteristic {
    zone_sectors: u32,
    max_open_zones: u32,
    max_active_zones: u32,
    max_append_sectors: u32,
    write_granularity: u32,
    model: u8,
    unused2: [u8; 3],
}

// SAFETY: Config only contains plain data types.
unsafe impl ByteValued for Config {}

// SAFETY: Geometry only contains plain data types.
unsafe impl ByteValued for Geometry {}

// SAFETY: ZoneCharacteristic only contains plain data types.
unsafe impl ByteValued for ZoneCharacteristic {}
