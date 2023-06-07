use vhost::vhost_user::message::VhostUserProtocolFeatures;
use vhost_user_backend::{VhostUserBackend, VhostUserDaemon, VringRwLock, VringState, VringT};
use vm_memory::{GuestMemory, GuestMemoryAtomic, GuestMemoryMmap};
use vmm_sys_util::epoll::EventSet;

/// An alias for `GuestMemoryAtomic<GuestMemoryMmap<B>>` to simplify code.
// TODO: fix (=export) in upstream (vhost-user-backend)
type GM<B> = GuestMemoryAtomic<GuestMemoryMmap<B>>;

fn main() {
    let mem = GuestMemoryAtomic::new(GuestMemoryMmap::new());
    let backend = Dummy;
    let daemon = VhostUserDaemon::<_, VringRwLock>::new("metis".into(), backend, mem);
}

#[derive(Debug, Clone)]
pub struct Dummy;

impl<V> VhostUserBackend<V> for Dummy
where
    V: VringT<GM<()>>,
{
    fn num_queues(&self) -> usize {
        todo!("num queues not yet implemented")
    }

    fn max_queue_size(&self) -> usize {
        todo!("max queue size not yet implemented")
    }

    fn features(&self) -> u64 {
        todo!("features not yet implemented")
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
