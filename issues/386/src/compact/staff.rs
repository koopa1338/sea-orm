//! SeaORM Entity. Generated by sea-orm-codegen 0.4.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "staff")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Custom(\"BLOB\".to_owned())"
    )]
    pub staff_id: String,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i32,
    #[sea_orm(column_type = "Custom(\"BLOB\".to_owned())", nullable)]
    pub picture: Option<String>,
    pub email: Option<String>,
    pub store_id: i32,
    #[sea_orm(column_type = "Custom(\"BLOB\".to_owned())")]
    pub active: String,
    pub username: String,
    pub password: Option<String>,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Address,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Store,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}