use vhost::vhost_user::{message::VhostUserProtocolFeatures, Listener};
use vhost_user_backend::{VhostUserBackend, VhostUserDaemon, VringRwLock, VringState, VringT};
use virtio_bindings::{
    virtio_blk::*, virtio_config::VIRTIO_F_VERSION_1, virtio_ring::VIRTIO_RING_F_EVENT_IDX,
};
use vm_memory::{GuestMemory, GuestMemoryAtomic, GuestMemoryMmap};
use vmm_sys_util::epoll::EventSet;

/// An alias for `GuestMemoryAtomic<GuestMemoryMmap<B>>` to simplify code.
// TODO: fix (=export) in upstream (vhost-user-backend)
type GM<B> = GuestMemoryAtomic<GuestMemoryMmap<B>>;

fn main() {
    let mem = GuestMemoryAtomic::new(GuestMemoryMmap::new());
    let backend = Dummy;
    let mut daemon = VhostUserDaemon::<_, VringRwLock>::new("metis".into(), backend, mem)
        .expect("can create vhost user daemon");
    daemon
        .start(Listener::new("./listener.sock", true).expect("can create vhost listener"))
        .expect("can start listening on vhost socket");
    daemon
        .wait()
        .expect("can wait until vhost daemon is finished");
}

#[derive(Debug, Clone)]
pub struct Dummy;

impl<V> VhostUserBackend<V> for Dummy
where
    V: VringT<GM<()>>,
{
    fn num_queues(&self) -> usize {
        // TODO
        1
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

    fn update_memory(&self, mem: GM<()>) -> std::io::Result<()> {
        todo!("update memory not yet implemented")
    }

    fn handle_event(
        &self,
        device_event: u16,
        evset: EventSet,
        vrings: &[V],
        thread_id: usize,
    ) -> std::io::Result<bool> {
        todo!("handle event not yet implemented")
    }
}
