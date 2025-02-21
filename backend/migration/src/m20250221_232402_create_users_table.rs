use sea_orm_migration::prelude::*;

/// The migration name is automatically derived from the file name.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // The `up` method applies the migration.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the "users" table.
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    // The `down` method reverts the migration.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the "users" table.
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

// Define identifiers for the table and its columns.
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
}
