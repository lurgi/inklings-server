use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(indexed)]
    pub user_id: i32,

    pub name: String,

    pub description: Option<String>,

    pub created_at: DateTime,

    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,

    #[sea_orm(has_many = "super::memo::Entity")]
    Memos,

    #[sea_orm(has_many = "super::essay::Entity")]
    Essays,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::memo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Memos.def()
    }
}

impl Related<super::essay::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Essays.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
