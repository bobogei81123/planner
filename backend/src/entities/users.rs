//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::iterations::Entity")]
    Iterations,
    #[sea_orm(has_many = "super::task_schedule::Entity")]
    TaskSchedule,
    #[sea_orm(has_many = "super::tasks::Entity")]
    Tasks,
}

impl Related<super::iterations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Iterations.def()
    }
}

impl Related<super::task_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskSchedule.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
