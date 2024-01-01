use std::{
    fmt,
    fs::File,
    io,
    os::{
        fd::AsRawFd,
        unix::prelude::{FileTypeExt, MetadataExt as _},
    },
};

use crate::kernel;

/// The layout of a given device describes low level IO characteristics.
///
/// Although this is intended primarily for block devices, it can also be used for files.
#[derive(Debug, Clone)]
pub struct Layout {
    /// Total size of the target in bytes.
    pub size: u64,
    /// Size of a logical block (sector) on the device.
    pub logical_block_size: u64,
    /// Size of a physical block on the device.
    pub physical_block_size: u64,
    /// The minimum size of an IO.
    pub minimum_io_size: u64,
    /// The optimal size of an IO. This is usually not reported and set to 0.
    pub optimal_io_size: u64,
}

/// An error encountered when loading the [`Layout`] of a device.
#[derive(Debug, Clone)]
pub enum LayoutError {
    /// Not a block device or regular file.
    UnsupportedDeviceType,
    /// IO error while querying metadata.
    IOError(io::ErrorKind),
    /// IO error while querying layout.
    QueryError(nix::Error),
}

impl Layout {
    pub fn new(target: &File) -> Result<Layout, LayoutError> {
        // Ioctls are only defined for block devices, so check if we have one of those first.
        let meta = target.metadata()?;
        if meta.file_type().is_block_device() {
            let fd = target.as_raw_fd();

            let mut size = 0;
            let mut physical_block_size = 0;
            let mut logical_block_size = 0;
            let mut minimum_io_size = 0;
            let mut optimal_io_size = 0;

            // SAFETY: ioctls on a valid file descriptor
            unsafe {
                kernel::ioctl_blkgetsize64(fd, &mut size as _)?;
                kernel::ioctl_blkpbszget(fd, &mut physical_block_size as _)?;
                kernel::ioctl_blksszget(fd, &mut logical_block_size as _)?;
                kernel::ioctl_blkiomin(fd, &mut minimum_io_size as _)?;
                kernel::ioctl_blkioopt(fd, &mut optimal_io_size as _)?;
            }

            Ok(Layout {
                size,
                logical_block_size: logical_block_size as _,
                physical_block_size: physical_block_size as _,
                minimum_io_size: minimum_io_size as _,
                optimal_io_size: optimal_io_size as _,
            })
        } else if meta.file_type().is_file() {
            // Fallback to reading some info from file metadata.
            Ok(Layout {
                size: meta.size(),
                logical_block_size: meta.blksize(),
                physical_block_size: meta.blksize(),
                // TODO: is this sufficient? or can this be queried?
                minimum_io_size: 512,
                optimal_io_size: 0,
            })
        } else {
            Err(LayoutError::UnsupportedDeviceType)
        }
    }
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutError::UnsupportedDeviceType => {
                f.write_str("target is not a block device or regular file")
            }
            LayoutError::IOError(kind) => f.write_fmt(format_args!(
                "i/o error {kind} while querying target metadata"
            )),
            LayoutError::QueryError(e) => f.write_fmt(format_args!(
                "i/o error {} while querying target metadata",
                e.desc()
            )),
        }
    }
}

impl std::error::Error for LayoutError {}

impl From<std::io::Error> for LayoutError {
    fn from(value: std::io::Error) -> Self {
        LayoutError::IOError(value.kind())
    }
}

impl From<nix::Error> for LayoutError {
    fn from(value: nix::Error) -> Self {
        LayoutError::QueryError(value)
    }
}
