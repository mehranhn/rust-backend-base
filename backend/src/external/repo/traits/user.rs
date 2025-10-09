pub trait ExternalRepoUser: super::ExternalRepoBase {
    async fn user_get_list(&self, tx: &mut Self::Connection);
    async fn user_get_by_id(&self, tx: &mut Self::Connection, id: uuid::Uuid);
    async fn user_create(&self, tx: &mut Self::Connection);
    async fn user_upsert(&self, tx: &mut Self::Connection);
    async fn user_update(&self, tx: &mut Self::Connection);
    async fn user_delete(&self, tx: &mut Self::Connection);
}
