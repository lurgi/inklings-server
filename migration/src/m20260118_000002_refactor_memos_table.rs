use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. project_id 컬럼 추가 (임시로 default 1)
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .add_column(
                        ColumnDef::new(Memos::ProjectId)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. project_id FK 추가
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk_memos_project_id")
                            .from_tbl(Memos::Table)
                            .from_col(Memos::ProjectId)
                            .to_tbl(Projects::Table)
                            .to_col(Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. project_id 인덱스 추가
        manager
            .create_index(
                Index::create()
                    .name("idx-memos-project_id")
                    .table(Memos::Table)
                    .col(Memos::ProjectId)
                    .to_owned(),
            )
            .await?;

        // 4. user_id FK 제거
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .drop_foreign_key(Alias::new("fk_memos_user_id"))
                    .to_owned(),
            )
            .await?;

        // 5. user_id 인덱스 제거
        manager
            .drop_index(
                Index::drop()
                    .name("idx-memos-user_id")
                    .table(Memos::Table)
                    .to_owned(),
            )
            .await?;

        // 6. user_id 컬럼 제거
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .drop_column(Memos::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rollback: user_id 복원
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .add_column(ColumnDef::new(Memos::UserId).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-memos-user_id")
                    .table(Memos::Table)
                    .col(Memos::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk_memos_user_id")
                            .from_tbl(Memos::Table)
                            .from_col(Memos::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // project_id FK 제거
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .drop_foreign_key(Alias::new("fk_memos_project_id"))
                    .to_owned(),
            )
            .await?;

        // project_id 인덱스 제거
        manager
            .drop_index(
                Index::drop()
                    .name("idx-memos-project_id")
                    .table(Memos::Table)
                    .to_owned(),
            )
            .await?;

        // project_id 컬럼 제거
        manager
            .alter_table(
                Table::alter()
                    .table(Memos::Table)
                    .drop_column(Memos::ProjectId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Memos {
    Table,
    UserId,
    ProjectId,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
