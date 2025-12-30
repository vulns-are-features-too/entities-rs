use std::fmt::Debug;
use typed_id::HasId;

/// Entity trait
///
/// ```
/// use entities::entity::Entity;
/// use typed_id::{Id, HasId};
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct User {
///     id: Id<Self>,
/// }
///
/// impl HasId<Self> for User {
///     fn id(&self) -> Id<Self> { self.id }
/// }
///
/// impl Entity for User {}
///
/// let alice = User { id: 1.into() };
/// let bob = User { id: 2.into() };
///
/// // Can compare IDs
/// assert_eq!(alice.id(), alice.id());
/// assert_ne!(alice.id(), bob.id());
/// assert!(alice.id() < bob.id());
///
/// // Can also compare directly, backed by IDs
/// assert_eq!(alice, alice);
/// assert_ne!(alice, bob);
/// assert!(alice < bob);
/// ```
///
/// ```compile_fail
/// use entities::entity::Entity;
/// use typed_id::{Id, HasId};
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct Table {
///     id: Id<Self>,
/// }
/// impl HasId<Self> for Table {
///     fn id(&self) -> Id<Self> { self.id }
/// }
/// impl Entity for Table {}
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct Chair {
///     id: Id<Self>,
/// }
/// impl HasId<Self> for Chair {
///     fn id(&self) -> Id<Self> { self.id }
/// }
/// impl Entity for Chair {}
///
/// let table = Table { id: 1.into() };
/// let chair = Chair { id: 1.into() };
///
/// // FAIL: Can't directly compare IDs of different types
/// assert_eq!(table.id(), chair.id());
/// ```
pub trait Entity: Debug + Eq + Ord + HasId<Self> + Sized {}
