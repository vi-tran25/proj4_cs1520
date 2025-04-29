use sea_orm_migration::{prelude::*, schema::*};

use super::m20220101_000001_create_category_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Purchase::Table)
                    .col(
                        ColumnDef::new(Purchase::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Purchase::Desc).string().not_null())
                    .col(ColumnDef::new(Purchase::Amount).integer().not_null())
                    .col(ColumnDef::new(Purchase::Date).string().not_null())
                    .col(ColumnDef::new(Purchase::CatID).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-purchase-category_id")
                            .from(Purchase::Table, Purchase::CatID)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Purchase::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Purchase {
    Table,
    Id,
    Desc,
    Amount,
    Date,
    CatID
}
