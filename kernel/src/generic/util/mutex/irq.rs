use crate::arch;
use core::{
    hint::likely,
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

per_cpu!(
    static IRQ_MUTEX: IrqMutex = IrqMutex {
        depth: AtomicUsize::new(0),
    };
);

pub struct IrqMutex {
    depth: AtomicUsize,
}

impl IrqMutex {
    pub fn lock<'a>() -> IrqGuard<'a> {
        unsafe { arch::irq::set_irq_state(false) };
        IRQ_MUTEX.get().depth.fetch_add(1, Ordering::Acquire);

        IrqGuard { _p: PhantomData }
    }
}

pub struct IrqGuard<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Drop for IrqGuard<'a> {
    fn drop(&mut self) {
        let old_depth = IRQ_MUTEX.get().depth.fetch_sub(1, Ordering::Acquire);

        // If the depth is now 0, re-enable IRQs.
        if likely(old_depth == 1) {
            unsafe { arch::irq::set_irq_state(true) };
        }
    }
}
