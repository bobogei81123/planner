//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "iterations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::iterations_tasks::Entity")]
    IterationsTasks,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::iterations_tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IterationsTasks.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        super::iterations_tasks::Relation::Tasks.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::iterations_tasks::Relation::Iterations.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
