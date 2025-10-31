mod user;
pub mod errors;

pub use user::ExRepoUser;

pub trait ExRepoUserModule: ExRepoUser {}
