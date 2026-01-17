use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MerchantAssets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantAssets::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantAssets::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantAssets::AssetType).text().not_null())
                    .col(ColumnDef::new(MerchantAssets::Name).text().not_null())
                    .col(ColumnDef::new(MerchantAssets::Status).text().not_null())
                    .col(
                        ColumnDef::new(MerchantAssets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MerchantAssets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_assets_merchant")
                            .from(MerchantAssets::Table, MerchantAssets::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(MerchantAssets::AssetType).is_in(vec![
                            "logo",
                            "brand",
                            "style",
                            "reference",
                        ]),
                    )
                    .check(Expr::col(MerchantAssets::Status).is_in(vec!["active", "archived"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantAssetVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantAssetVersions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MerchantAssetVersions::AssetId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MerchantAssetVersions::Version)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MerchantAssetVersions::Payload)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MerchantAssetVersions::CreatedBy)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MerchantAssetVersions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_asset_versions_asset")
                            .from(MerchantAssetVersions::Table, MerchantAssetVersions::AssetId)
                            .to(MerchantAssets::Table, MerchantAssets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_merchant_assets_merchant_type_status")
                    .table(MerchantAssets::Table)
                    .col(MerchantAssets::MerchantId)
                    .col(MerchantAssets::AssetType)
                    .col(MerchantAssets::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uk_merchant_asset_versions_asset_version")
                    .table(MerchantAssetVersions::Table)
                    .col(MerchantAssetVersions::AssetId)
                    .col(MerchantAssetVersions::Version)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MerchantAssetVersions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantAssets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum MerchantAssets {
    Table,
    Id,
    MerchantId,
    AssetType,
    Name,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum MerchantAssetVersions {
    Table,
    Id,
    AssetId,
    Version,
    Payload,
    CreatedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Merchants {
    Table,
    Id,
}
