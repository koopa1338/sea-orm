//! SeaORM Entity. Generated by sea-orm-codegen 0.4.2

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "category"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub category_id: String,
    pub name: String,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CategoryId,
    Name,
    LastUpdate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CategoryId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    FilmCategory,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CategoryId => ColumnType::Custom("BLOB".to_owned()).def(),
            Self::Name => ColumnType::String(None).def(),
            Self::LastUpdate => ColumnType::Timestamp.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::FilmCategory => Entity::has_many(super::film_category::Entity).into(),
        }
    }
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}