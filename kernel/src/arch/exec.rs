use super::internal;
use crate::generic::sched::task::Frame;

pub use internal::task::TaskFrame;
assert_trait_impl!(TaskFrame, Frame);
