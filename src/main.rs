use config::Config;
use log::info;
use vhost::vhost_user::{message::VhostUserProtocolFeatures, Listener};
use vhost_user_backend::{VhostUserBackend, VhostUserDaemon, VringRwLock};
use virtio_bindings::{
    virtio_blk::*, virtio_config::VIRTIO_F_VERSION_1, virtio_ring::VIRTIO_RING_F_EVENT_IDX,
};
use vm_memory::{bitmap::AtomicBitmap, ByteValued, GuestMemoryAtomic, GuestMemoryMmap};
use vmm_sys_util::epoll::EventSet;

mod config;

/// An alias for `GuestMemoryAtomic<GuestMemoryMmap<B>>` to simplify code.
// TODO: fix (=export) in upstream (vhost-user-backend)
type GM<B> = GuestMemoryAtomic<GuestMemoryMmap<B>>;

fn main() {
    let mem = GuestMemoryAtomic::new(GuestMemoryMmap::new());
    let backend = BlockBackend {
        config: Default::default(),
    };
    let mut daemon =
        VhostUserDaemon::new("metis".into(), backend, mem).expect("can create vhost user daemon");
    daemon
        .start(Listener::new("./listener.sock", true).expect("can create vhost listener"))
        .expect("can start listening on vhost socket");
    daemon
        .wait()
        .expect("can wait until vhost daemon is finished");
}

#[derive(Debug, Clone)]
pub struct BlockBackend {
    config: Config,
}

impl VhostUserBackend for BlockBackend {
    type Bitmap = AtomicBitmap;
    type Vring = VringRwLock<GuestMemoryAtomic<GuestMemoryMmap<AtomicBitmap>>>;

    fn num_queues(&self) -> usize {
        self.config.num_queues as usize
    }

    fn max_queue_size(&self) -> usize {
        // TODO
        128
    }

    fn features(&self) -> u64 {
        // TODO
        1 << VIRTIO_BLK_F_SIZE_MAX // max segment size
            | 1 << VIRTIO_BLK_F_SEG_MAX // max nr of segments
            | 1 << VIRTIO_BLK_F_GEOMETRY // legacy geometry
            | 1 << VIRTIO_BLK_F_BLK_SIZE // block size available
            | 1 << VIRTIO_BLK_F_CONFIG_WCE // enable writeback mode in config 
            | 1 << VIRTIO_BLK_F_MQ // support more than one virtqueue
            | 1 << VIRTIO_BLK_F_WRITE_ZEROES // WRITE_ZEROES supported
            | 1 << VIRTIO_RING_F_EVENT_IDX // guest sets index for which it wants an interrupt
            | 1 << VIRTIO_F_VERSION_1 // compatbile with V1 virtio
            | 1 << VIRTIO_BLK_F_FLUSH // flush support -> legacy
    }

    fn protocol_features(&self) -> VhostUserProtocolFeatures {
        todo!("protocol features not yet impelmentend")
    }

    fn set_event_idx(&self, enabled: bool) {
        todo!("set event idx not yet implemented")
    }

    fn update_memory(&self, mem: GM<Self::Bitmap>) -> std::io::Result<()> {
        todo!()
    }

    fn handle_event(
        &self,
        device_event: u16,
        evset: EventSet,
        vrings: &[Self::Vring],
        thread_id: usize,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn acked_features(&self, _features: u64) {}

    fn get_config(&self, offset: u32, size: u32) -> Vec<u8> {
        info!("Requesting virtio device config at offset {offset} with size {size}");
        self.config.as_slice().to_vec()
    }

    fn set_config(&self, _offset: u32, _buf: &[u8]) -> std::io::Result<()> {
        Ok(())
    }

    fn set_backend_req_fd(&self, _backend: vhost::vhost_user::Backend) {}

    fn queues_per_thread(&self) -> Vec<u64> {
        vec![0xffff_ffff]
    }

    fn exit_event(&self, _thread_index: usize) -> Option<vmm_sys_util::eventfd::EventFd> {
        None
    }
}
