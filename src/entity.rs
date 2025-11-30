use std::fmt::Debug;
use typed_id::HasId;

pub trait Entity<T>: Debug + HasId<T> {}
