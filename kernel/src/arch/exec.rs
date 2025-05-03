use super::internal;
use crate::generic::exec::Frame;

pub use internal::TaskFrame;
assert_trait_impl!(TaskFrame, Frame);
