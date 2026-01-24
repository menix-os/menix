use crate::arch;
use core::{
    hint::likely,
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

per_cpu!(
    static IRQ_MUTEX: IrqLock = IrqLock {
        depth: AtomicUsize::new(0),
        in_interrupt: AtomicBool::new(false),
    };
);

pub struct IrqLock {
    depth: AtomicUsize,
    in_interrupt: AtomicBool,
}

impl IrqLock {
    pub fn lock<'a>() -> IrqGuard<'a> {
        let cpu = IRQ_MUTEX.get();

        if !cpu.in_interrupt.load(Ordering::Acquire) {
            unsafe { arch::irq::set_irq_state(false) };
            cpu.depth.fetch_add(1, Ordering::Acquire);
        }

        IrqGuard { _p: PhantomData }
    }

    pub fn set_interrupted(value: bool) -> bool {
        IRQ_MUTEX.get().in_interrupt.swap(value, Ordering::Release)
    }
}

pub struct IrqGuard<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Drop for IrqGuard<'a> {
    fn drop(&mut self) {
        let cpu = IRQ_MUTEX.get();
        if !cpu.in_interrupt.load(Ordering::Acquire) {
            let old_depth = cpu.depth.fetch_sub(1, Ordering::Acquire);

            // If the depth is now 0, re-enable IRQs.
            if likely(old_depth == 1) {
                unsafe { arch::irq::set_irq_state(true) };
            }
        }
    }
}
