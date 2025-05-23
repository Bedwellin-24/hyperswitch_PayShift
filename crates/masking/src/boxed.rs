//! `Box` types containing secrets
//!
//! There is not alias type by design.

#[cfg(feature = "serde")]
use super::{SerializableSecret, Serialize};

#[cfg(feature = "serde")]
impl<S: Serialize> SerializableSecret for Box<S> {}
