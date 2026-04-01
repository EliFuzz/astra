#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub trait StorageBounds: Send + Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send + Sync> StorageBounds for T {}

#[cfg(target_arch = "wasm32")]
pub trait StorageBounds {}

#[cfg(target_arch = "wasm32")]
impl<T> StorageBounds for T {}
