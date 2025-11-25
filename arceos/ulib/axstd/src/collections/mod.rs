#[cfg(feature = "alloc")]
pub mod hashmap;
#[cfg(feature = "alloc")]
pub use self::hashmap::HashMap;