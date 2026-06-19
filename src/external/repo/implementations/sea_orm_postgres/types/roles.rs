use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "roles_enum")]
pub enum Roles {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Salesmen")]
    Salesmen,
}
