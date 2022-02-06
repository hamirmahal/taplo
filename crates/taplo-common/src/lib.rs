pub mod config;
pub mod environment;
pub mod schema;
pub mod util;

pub type HashMap<K, V> = ahash::AHashMap<K, V>;
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, ahash::RandomState>;

pub type AsyncMutex<T> = tokio::sync::Mutex<T>;
pub type AsyncRwLock<T> = tokio::sync::RwLock<T>;