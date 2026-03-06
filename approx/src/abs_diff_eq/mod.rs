mod collections;
mod core_impls;
mod external;
mod primitives;
mod trait_def;
#[cfg(feature = "tuple_impl")]
mod tuples;

pub use trait_def::AbsDiffEq;
