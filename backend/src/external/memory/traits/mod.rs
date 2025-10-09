mod phone;

pub use phone::ExternalMemoryPhone;

pub trait ExternalMemory: ExternalMemoryPhone + Send + Sync + 'static {
}
