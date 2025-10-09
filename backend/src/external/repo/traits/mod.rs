mod user;

pub use user::ExternalRepoUser;

pub trait ExternalRepoTx {
    async fn rollback(self);
    async fn commit(self);
}

pub trait ExternalRepoBase: Send + Sync + 'static {
    type Connection;
    type Tx: ExternalRepoTx + AsMut<Self::Connection>;

    async fn connection(&self) -> Self::Connection;
    async fn transaction(&self) -> Self::Tx;
}


pub trait ExternalRepo: ExternalRepoUser { }
