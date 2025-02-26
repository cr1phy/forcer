use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string_uniq(User::Email))
                    .col(string_len_uniq(User::Username, 128))
                    .col(binary(User::Password))
                    .col(date_time(User::CreatedAt))
                    .col(date_time(User::LastOnline))
                    .col(boolean(User::IsActive))
                    .col(boolean(User::IsVerified))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    CreatedAt,
    LastOnline,
    IsActive,
    IsVerified,
}