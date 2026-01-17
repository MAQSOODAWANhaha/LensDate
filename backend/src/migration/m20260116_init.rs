use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Phone).string_len(32).not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string_len(255).unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).text())
                    .col(ColumnDef::new(Users::Status).text().not_null())
                    .col(
                        ColumnDef::new(Users::CreditScore)
                            .integer()
                            .not_null()
                            .default(Expr::value(100)),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Users::DeletedAt).timestamp_with_time_zone())
                    .check(
                        Expr::col(Users::Status).is_in(vec!["active", "frozen", "deleted"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserProfiles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfiles::UserId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserProfiles::Nickname).string_len(64))
                    .col(ColumnDef::new(UserProfiles::AvatarUrl).text())
                    .col(ColumnDef::new(UserProfiles::Gender).text())
                    .col(ColumnDef::new(UserProfiles::Birthday).date())
                    .col(ColumnDef::new(UserProfiles::CityId).big_integer())
                    .col(ColumnDef::new(UserProfiles::Bio).text())
                    .col(
                        ColumnDef::new(UserProfiles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_profiles_user")
                            .from(UserProfiles::Table, UserProfiles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(UserProfiles::Gender)
                            .is_in(vec!["male", "female", "unknown"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).text().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).big_integer().not_null())
                    .col(ColumnDef::new(UserRoles::Scope).text())
                    .primary_key(
                        Index::create()
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_role")
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sessions::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Sessions::Token).text().not_null().unique_key())
                    .col(ColumnDef::new(Sessions::ExpiredAt).timestamp_with_time_zone().not_null())
                    .col(
                        ColumnDef::new(Sessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_user")
                            .from(Sessions::Table, Sessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(VerificationCodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VerificationCodes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VerificationCodes::Phone).string_len(32).not_null())
                    .col(ColumnDef::new(VerificationCodes::Code).string_len(16).not_null())
                    .col(
                        ColumnDef::new(VerificationCodes::ExpiredAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VerificationCodes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Photographers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Photographers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Photographers::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Photographers::Type).text().not_null())
                    .col(ColumnDef::new(Photographers::Status).text().not_null())
                    .col(ColumnDef::new(Photographers::CityId).big_integer())
                    .col(ColumnDef::new(Photographers::ServiceArea).text())
                    .col(
                        ColumnDef::new(Photographers::RatingAvg)
                            .decimal_len(3, 2)
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .col(
                        ColumnDef::new(Photographers::CompletedOrders)
                            .integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .col(
                        ColumnDef::new(Photographers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Photographers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_photographers_user")
                            .from(Photographers::Table, Photographers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(Photographers::Type).is_in(vec!["individual", "team"]))
                    .check(
                        Expr::col(Photographers::Status)
                            .is_in(vec!["pending", "approved", "rejected", "frozen"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teams::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teams::OwnerUserId).big_integer().not_null())
                    .col(ColumnDef::new(Teams::Name).text().not_null())
                    .col(ColumnDef::new(Teams::Status).text().not_null())
                    .col(
                        ColumnDef::new(Teams::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Teams::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teams_owner")
                            .from(Teams::Table, Teams::OwnerUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(Teams::Status).is_in(vec!["active", "frozen", "deleted"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamMembers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TeamMembers::TeamId).big_integer().not_null())
                    .col(ColumnDef::new(TeamMembers::UserId).big_integer().not_null())
                    .col(ColumnDef::new(TeamMembers::Role).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(TeamMembers::TeamId)
                            .col(TeamMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_team_members_team")
                            .from(TeamMembers::Table, TeamMembers::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_team_members_user")
                            .from(TeamMembers::Table, TeamMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(TeamMembers::Role).is_in(vec!["admin", "member"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Portfolios::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Portfolios::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Portfolios::PhotographerId).big_integer().not_null())
                    .col(ColumnDef::new(Portfolios::Title).text().not_null())
                    .col(ColumnDef::new(Portfolios::Status).text().not_null())
                    .col(
                        ColumnDef::new(Portfolios::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Portfolios::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_portfolios_photographer")
                            .from(Portfolios::Table, Portfolios::PhotographerId)
                            .to(Photographers::Table, Photographers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Portfolios::Status)
                            .is_in(vec!["draft", "published", "blocked"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PortfolioItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PortfolioItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PortfolioItems::PortfolioId).big_integer().not_null())
                    .col(ColumnDef::new(PortfolioItems::Url).text().not_null())
                    .col(ColumnDef::new(PortfolioItems::Tags).json_binary())
                    .col(
                        ColumnDef::new(PortfolioItems::CoverFlag)
                            .boolean()
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .col(
                        ColumnDef::new(PortfolioItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_portfolio_items_portfolio")
                            .from(PortfolioItems::Table, PortfolioItems::PortfolioId)
                            .to(Portfolios::Table, Portfolios::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Demands::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Demands::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Demands::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Demands::Type).text().not_null())
                    .col(ColumnDef::new(Demands::CityId).big_integer())
                    .col(ColumnDef::new(Demands::Location).text())
                    .col(ColumnDef::new(Demands::ScheduleStart).timestamp_with_time_zone())
                    .col(ColumnDef::new(Demands::ScheduleEnd).timestamp_with_time_zone())
                    .col(ColumnDef::new(Demands::BudgetMin).decimal_len(12, 2))
                    .col(ColumnDef::new(Demands::BudgetMax).decimal_len(12, 2))
                    .col(ColumnDef::new(Demands::PeopleCount).integer())
                    .col(ColumnDef::new(Demands::StyleTags).json_binary())
                    .col(ColumnDef::new(Demands::Status).text().not_null())
                    .col(
                        ColumnDef::new(Demands::IsMerchant)
                            .boolean()
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .col(ColumnDef::new(Demands::MerchantId).big_integer())
                    .col(
                        ColumnDef::new(Demands::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Demands::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_demands_user")
                            .from(Demands::Table, Demands::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(Demands::Status).is_in(vec!["draft", "open", "closed"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DemandAttachments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DemandAttachments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DemandAttachments::DemandId).big_integer().not_null())
                    .col(ColumnDef::new(DemandAttachments::FileUrl).text().not_null())
                    .col(ColumnDef::new(DemandAttachments::FileType).text())
                    .col(
                        ColumnDef::new(DemandAttachments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_demand_attachments_demand")
                            .from(DemandAttachments::Table, DemandAttachments::DemandId)
                            .to(Demands::Table, Demands::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Quotes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quotes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Quotes::DemandId).big_integer().not_null())
                    .col(ColumnDef::new(Quotes::PhotographerId).big_integer())
                    .col(ColumnDef::new(Quotes::TeamId).big_integer())
                    .col(ColumnDef::new(Quotes::TotalPrice).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Quotes::Status).text().not_null())
                    .col(
                        ColumnDef::new(Quotes::Version)
                            .integer()
                            .not_null()
                            .default(Expr::value(1)),
                    )
                    .col(ColumnDef::new(Quotes::ExpiresAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Quotes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Quotes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quotes_demand")
                            .from(Quotes::Table, Quotes::DemandId)
                            .to(Demands::Table, Demands::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Quotes::Status).is_in(vec!["pending", "accepted", "expired"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuoteItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QuoteItems::QuoteId).big_integer().not_null())
                    .col(ColumnDef::new(QuoteItems::Name).text().not_null())
                    .col(ColumnDef::new(QuoteItems::Price).decimal_len(12, 2).not_null())
                    .col(
                        ColumnDef::new(QuoteItems::Quantity)
                            .integer()
                            .not_null()
                            .default(Expr::value(1)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_items_quote")
                            .from(QuoteItems::Table, QuoteItems::QuoteId)
                            .to(Quotes::Table, Quotes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuoteVersions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QuoteVersions::QuoteId).big_integer().not_null())
                    .col(ColumnDef::new(QuoteVersions::Version).integer().not_null())
                    .col(
                        ColumnDef::new(QuoteVersions::TotalPrice)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(QuoteVersions::Items).json_binary().not_null())
                    .col(ColumnDef::new(QuoteVersions::Note).text())
                    .col(ColumnDef::new(QuoteVersions::CreatedBy).big_integer().not_null())
                    .col(
                        ColumnDef::new(QuoteVersions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_versions_quote")
                            .from(QuoteVersions::Table, QuoteVersions::QuoteId)
                            .to(Quotes::Table, Quotes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quote_versions_user")
                            .from(QuoteVersions::Table, QuoteVersions::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Orders::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Orders::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Orders::PhotographerId).big_integer())
                    .col(ColumnDef::new(Orders::TeamId).big_integer())
                    .col(ColumnDef::new(Orders::DemandId).big_integer())
                    .col(ColumnDef::new(Orders::QuoteId).big_integer())
                    .col(ColumnDef::new(Orders::Status).text().not_null())
                    .col(ColumnDef::new(Orders::PayType).text().not_null())
                    .col(
                        ColumnDef::new(Orders::DepositAmount)
                            .decimal_len(12, 2)
                            .default(Expr::value(0)),
                    )
                    .col(ColumnDef::new(Orders::TotalAmount).decimal_len(12, 2).not_null())
                    .col(
                        ColumnDef::new(Orders::ServiceFee)
                            .decimal_len(12, 2)
                            .default(Expr::value(0)),
                    )
                    .col(ColumnDef::new(Orders::ScheduleStart).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::ScheduleEnd).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::CancelledAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Orders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Orders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_user")
                            .from(Orders::Table, Orders::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_demand")
                            .from(Orders::Table, Orders::DemandId)
                            .to(Demands::Table, Demands::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_quote")
                            .from(Orders::Table, Orders::QuoteId)
                            .to(Quotes::Table, Quotes::Id),
                    )
                    .check(
                        Expr::col(Orders::Status).is_in(vec![
                            "confirmed",
                            "paid",
                            "ongoing",
                            "completed",
                            "reviewed",
                            "cancelled",
                        ]),
                    )
                    .check(
                        Expr::col(Orders::PayType).is_in(vec!["deposit", "full", "phase"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItems::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(OrderItems::Name).text().not_null())
                    .col(ColumnDef::new(OrderItems::Price).decimal_len(12, 2).not_null())
                    .col(
                        ColumnDef::new(OrderItems::Quantity)
                            .integer()
                            .not_null()
                            .default(Expr::value(1)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_items_order")
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Payments::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(Payments::PayerId).big_integer().not_null())
                    .col(ColumnDef::new(Payments::PayeeId).big_integer().not_null())
                    .col(ColumnDef::new(Payments::Amount).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Payments::Status).text().not_null())
                    .col(ColumnDef::new(Payments::PayChannel).text().not_null())
                    .col(ColumnDef::new(Payments::Stage).text())
                    .col(ColumnDef::new(Payments::PaidAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Payments::ProofUrl).text())
                    .col(
                        ColumnDef::new(Payments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payments_order")
                            .from(Payments::Table, Payments::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payments_payer")
                            .from(Payments::Table, Payments::PayerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payments_payee")
                            .from(Payments::Table, Payments::PayeeId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Payments::Status)
                            .is_in(vec!["pending", "success", "failed"]),
                    )
                    .check(
                        Expr::col(Payments::PayChannel)
                            .is_in(vec!["wx", "alipay", "bank"]),
                    )
                    .check(
                        Expr::col(Payments::Stage)
                            .is_in(vec!["deposit", "mid", "final"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Refunds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Refunds::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Refunds::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(Refunds::ApplicantId).big_integer().not_null())
                    .col(ColumnDef::new(Refunds::Amount).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Refunds::Status).text().not_null())
                    .col(ColumnDef::new(Refunds::ResponsibleParty).text())
                    .col(ColumnDef::new(Refunds::Reason).text())
                    .col(ColumnDef::new(Refunds::ProofUrl).text())
                    .col(
                        ColumnDef::new(Refunds::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Refunds::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refunds_order")
                            .from(Refunds::Table, Refunds::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refunds_applicant")
                            .from(Refunds::Table, Refunds::ApplicantId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Refunds::Status)
                            .is_in(vec!["pending", "approved", "rejected", "paid"]),
                    )
                    .check(
                        Expr::col(Refunds::ResponsibleParty)
                            .is_in(vec!["user", "photographer", "merchant"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Deliveries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deliveries::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Deliveries::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(Deliveries::Status).text().not_null())
                    .col(ColumnDef::new(Deliveries::SubmittedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Deliveries::AcceptedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_deliveries_order")
                            .from(Deliveries::Table, Deliveries::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Deliveries::Status)
                            .is_in(vec!["pending", "submitted", "accepted", "rejected"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DeliveryItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeliveryItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DeliveryItems::DeliveryId).big_integer().not_null())
                    .col(ColumnDef::new(DeliveryItems::FileUrl).text().not_null())
                    .col(ColumnDef::new(DeliveryItems::Version).text())
                    .col(ColumnDef::new(DeliveryItems::Note).text())
                    .col(
                        ColumnDef::new(DeliveryItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_delivery_items_delivery")
                            .from(DeliveryItems::Table, DeliveryItems::DeliveryId)
                            .to(Deliveries::Table, Deliveries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reviews::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(Reviews::RaterId).big_integer().not_null())
                    .col(ColumnDef::new(Reviews::RateeId).big_integer().not_null())
                    .col(ColumnDef::new(Reviews::Score).integer().not_null())
                    .col(ColumnDef::new(Reviews::Tags).json_binary())
                    .col(ColumnDef::new(Reviews::Comment).text())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_order")
                            .from(Reviews::Table, Reviews::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_rater")
                            .from(Reviews::Table, Reviews::RaterId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_ratee")
                            .from(Reviews::Table, Reviews::RateeId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(Reviews::Score).between(1, 5))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Disputes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Disputes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Disputes::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(Disputes::InitiatorId).big_integer().not_null())
                    .col(ColumnDef::new(Disputes::Status).text().not_null())
                    .col(ColumnDef::new(Disputes::Reason).text())
                    .col(ColumnDef::new(Disputes::Resolution).text())
                    .col(
                        ColumnDef::new(Disputes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Disputes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_disputes_order")
                            .from(Disputes::Table, Disputes::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_disputes_initiator")
                            .from(Disputes::Table, Disputes::InitiatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(Disputes::Status)
                            .is_in(vec!["submitted", "handling", "closed"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DisputeEvidence::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DisputeEvidence::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DisputeEvidence::DisputeId).big_integer().not_null())
                    .col(ColumnDef::new(DisputeEvidence::FileUrl).text().not_null())
                    .col(ColumnDef::new(DisputeEvidence::Note).text())
                    .col(
                        ColumnDef::new(DisputeEvidence::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dispute_evidence_dispute")
                            .from(DisputeEvidence::Table, DisputeEvidence::DisputeId)
                            .to(Disputes::Table, Disputes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Merchants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Merchants::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Merchants::Name).text().not_null())
                    .col(ColumnDef::new(Merchants::LogoUrl).text())
                    .col(ColumnDef::new(Merchants::BrandColor).text())
                    .col(ColumnDef::new(Merchants::ContactUserId).big_integer())
                    .col(ColumnDef::new(Merchants::Status).text().not_null())
                    .col(
                        ColumnDef::new(Merchants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Merchants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchants_contact")
                            .from(Merchants::Table, Merchants::ContactUserId)
                            .to(Users::Table, Users::Id),
                    )
                    .check(
                        Expr::col(Merchants::Status).is_in(vec!["pending", "approved", "frozen"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantLocations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantLocations::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantLocations::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantLocations::Name).text().not_null())
                    .col(ColumnDef::new(MerchantLocations::Address).text())
                    .col(ColumnDef::new(MerchantLocations::CityId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_locations_merchant")
                            .from(MerchantLocations::Table, MerchantLocations::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantUsers::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantUsers::UserId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantUsers::Role).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_users_merchant")
                            .from(MerchantUsers::Table, MerchantUsers::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_users_user")
                            .from(MerchantUsers::Table, MerchantUsers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(MerchantUsers::Role)
                            .is_in(vec!["requester", "approver", "finance"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantTemplates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantTemplates::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantTemplates::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantTemplates::Name).text().not_null())
                    .col(ColumnDef::new(MerchantTemplates::Description).text())
                    .col(ColumnDef::new(MerchantTemplates::DeliveryRequirements).json_binary())
                    .col(
                        ColumnDef::new(MerchantTemplates::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_templates_merchant")
                            .from(MerchantTemplates::Table, MerchantTemplates::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantTemplateItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantTemplateItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantTemplateItems::TemplateId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantTemplateItems::Name).text().not_null())
                    .col(
                        ColumnDef::new(MerchantTemplateItems::Quantity)
                            .integer()
                            .not_null()
                            .default(Expr::value(1)),
                    )
                    .col(
                        ColumnDef::new(MerchantTemplateItems::Price)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_template_items_template")
                            .from(MerchantTemplateItems::Table, MerchantTemplateItems::TemplateId)
                            .to(MerchantTemplates::Table, MerchantTemplates::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantContracts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantContracts::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantContracts::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantContracts::Terms).json_binary().not_null())
                    .col(
                        ColumnDef::new(MerchantContracts::Version)
                            .integer()
                            .not_null()
                            .default(Expr::value(1)),
                    )
                    .col(
                        ColumnDef::new(MerchantContracts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_contracts_order")
                            .from(MerchantContracts::Table, MerchantContracts::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantInvoices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantInvoices::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantInvoices::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantInvoices::OrderId).big_integer())
                    .col(ColumnDef::new(MerchantInvoices::Title).text().not_null())
                    .col(ColumnDef::new(MerchantInvoices::TaxNo).text())
                    .col(
                        ColumnDef::new(MerchantInvoices::Amount)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(MerchantInvoices::Status).text().not_null())
                    .col(
                        ColumnDef::new(MerchantInvoices::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_invoices_merchant")
                            .from(MerchantInvoices::Table, MerchantInvoices::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_invoices_order")
                            .from(MerchantInvoices::Table, MerchantInvoices::OrderId)
                            .to(Orders::Table, Orders::Id),
                    )
                    .check(
                        Expr::col(MerchantInvoices::Status)
                            .is_in(vec!["pending", "issued", "rejected"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MerchantApprovals::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerchantApprovals::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerchantApprovals::DemandId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantApprovals::MerchantId).big_integer().not_null())
                    .col(ColumnDef::new(MerchantApprovals::Status).text().not_null())
                    .col(ColumnDef::new(MerchantApprovals::ApproverId).big_integer())
                    .col(ColumnDef::new(MerchantApprovals::Comment).text())
                    .col(
                        ColumnDef::new(MerchantApprovals::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_approvals_demand")
                            .from(MerchantApprovals::Table, MerchantApprovals::DemandId)
                            .to(Demands::Table, Demands::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_approvals_merchant")
                            .from(MerchantApprovals::Table, MerchantApprovals::MerchantId)
                            .to(Merchants::Table, Merchants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_approvals_approver")
                            .from(MerchantApprovals::Table, MerchantApprovals::ApproverId)
                            .to(Users::Table, Users::Id),
                    )
                    .check(
                        Expr::col(MerchantApprovals::Status)
                            .is_in(vec!["draft", "pending", "approved", "rejected"]),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Conversations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conversations::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Conversations::Type).text().not_null())
                    .col(ColumnDef::new(Conversations::OrderId).big_integer())
                    .col(
                        ColumnDef::new(Conversations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conversations_order")
                            .from(Conversations::Table, Conversations::OrderId)
                            .to(Orders::Table, Orders::Id),
                    )
                    .check(Expr::col(Conversations::Type).is_in(vec!["order", "chat"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Messages::ConversationId).big_integer().not_null())
                    .col(ColumnDef::new(Messages::SenderId).big_integer().not_null())
                    .col(ColumnDef::new(Messages::Content).text())
                    .col(ColumnDef::new(Messages::MsgType).text().not_null())
                    .col(
                        ColumnDef::new(Messages::SentAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_conversation")
                            .from(Messages::Table, Messages::ConversationId)
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_sender")
                            .from(Messages::Table, Messages::SenderId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(Expr::col(Messages::MsgType).is_in(vec!["text", "image", "file"]))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notifications::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notifications::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Notifications::Type).text().not_null())
                    .col(ColumnDef::new(Notifications::Title).text())
                    .col(ColumnDef::new(Notifications::Content).text())
                    .col(ColumnDef::new(Notifications::ReadAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Notifications::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_notifications_user")
                            .from(Notifications::Table, Notifications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditLogs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditLogs::AdminId).big_integer().not_null())
                    .col(ColumnDef::new(AuditLogs::Action).text().not_null())
                    .col(ColumnDef::new(AuditLogs::TargetType).text())
                    .col(ColumnDef::new(AuditLogs::TargetId).big_integer())
                    .col(ColumnDef::new(AuditLogs::Detail).json_binary())
                    .col(
                        ColumnDef::new(AuditLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_logs_admin")
                            .from(AuditLogs::Table, AuditLogs::AdminId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Configs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Configs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Configs::Key).text().not_null().unique_key())
                    .col(ColumnDef::new(Configs::Value).json_binary().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_demands_city_status_start")
                    .table(Demands::Table)
                    .col(Demands::CityId)
                    .col(Demands::Status)
                    .col(Demands::ScheduleStart)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_orders_user_status_created")
                    .table(Orders::Table)
                    .col(Orders::UserId)
                    .col(Orders::Status)
                    .col(Orders::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_messages_conversation_sent")
                    .table(Messages::Table)
                    .col(Messages::ConversationId)
                    .col(Messages::SentAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_quote_versions_quote_version")
                    .table(QuoteVersions::Table)
                    .col(QuoteVersions::QuoteId)
                    .col(QuoteVersions::Version)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Configs::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Notifications::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Messages::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Conversations::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantApprovals::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantInvoices::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantContracts::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantTemplateItems::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantTemplates::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantUsers::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MerchantLocations::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Merchants::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DisputeEvidence::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Disputes::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reviews::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DeliveryItems::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Deliveries::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Refunds::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Payments::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderItems::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Orders::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuoteVersions::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuoteItems::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Quotes::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DemandAttachments::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Demands::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PortfolioItems::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Portfolios::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TeamMembers::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Teams::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Photographers::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(VerificationCodes::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Sessions::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserRoles::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserProfiles::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Phone,
    Email,
    PasswordHash,
    Status,
    CreditScore,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum UserProfiles {
    Table,
    UserId,
    Nickname,
    AvatarUrl,
    Gender,
    Birthday,
    CityId,
    Bio,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
    Scope,
}

#[derive(Iden)]
enum Sessions {
    Table,
    Id,
    UserId,
    Token,
    ExpiredAt,
    CreatedAt,
}

#[derive(Iden)]
enum VerificationCodes {
    Table,
    Id,
    Phone,
    Code,
    ExpiredAt,
    CreatedAt,
}

#[derive(Iden)]
enum Photographers {
    Table,
    Id,
    UserId,
    Type,
    Status,
    CityId,
    ServiceArea,
    RatingAvg,
    CompletedOrders,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Teams {
    Table,
    Id,
    OwnerUserId,
    Name,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum TeamMembers {
    Table,
    TeamId,
    UserId,
    Role,
}

#[derive(Iden)]
enum Portfolios {
    Table,
    Id,
    PhotographerId,
    Title,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PortfolioItems {
    Table,
    Id,
    PortfolioId,
    Url,
    Tags,
    CoverFlag,
    CreatedAt,
}

#[derive(Iden)]
enum Demands {
    Table,
    Id,
    UserId,
    Type,
    CityId,
    Location,
    ScheduleStart,
    ScheduleEnd,
    BudgetMin,
    BudgetMax,
    PeopleCount,
    StyleTags,
    Status,
    IsMerchant,
    MerchantId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum DemandAttachments {
    Table,
    Id,
    DemandId,
    FileUrl,
    FileType,
    CreatedAt,
}

#[derive(Iden)]
enum Quotes {
    Table,
    Id,
    DemandId,
    PhotographerId,
    TeamId,
    TotalPrice,
    Status,
    Version,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum QuoteItems {
    Table,
    Id,
    QuoteId,
    Name,
    Price,
    Quantity,
}

#[derive(Iden)]
enum QuoteVersions {
    Table,
    Id,
    QuoteId,
    Version,
    TotalPrice,
    Items,
    Note,
    CreatedBy,
    CreatedAt,
}

#[derive(Iden)]
enum Orders {
    Table,
    Id,
    UserId,
    PhotographerId,
    TeamId,
    DemandId,
    QuoteId,
    Status,
    PayType,
    DepositAmount,
    TotalAmount,
    ServiceFee,
    ScheduleStart,
    ScheduleEnd,
    CancelledAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    Name,
    Price,
    Quantity,
}

#[derive(Iden)]
enum Payments {
    Table,
    Id,
    OrderId,
    PayerId,
    PayeeId,
    Amount,
    Status,
    PayChannel,
    Stage,
    PaidAt,
    ProofUrl,
    CreatedAt,
}

#[derive(Iden)]
enum Refunds {
    Table,
    Id,
    OrderId,
    ApplicantId,
    Amount,
    Status,
    ResponsibleParty,
    Reason,
    ProofUrl,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Deliveries {
    Table,
    Id,
    OrderId,
    Status,
    SubmittedAt,
    AcceptedAt,
}

#[derive(Iden)]
enum DeliveryItems {
    Table,
    Id,
    DeliveryId,
    FileUrl,
    Version,
    Note,
    CreatedAt,
}

#[derive(Iden)]
enum Reviews {
    Table,
    Id,
    OrderId,
    RaterId,
    RateeId,
    Score,
    Tags,
    Comment,
    CreatedAt,
}

#[derive(Iden)]
enum Disputes {
    Table,
    Id,
    OrderId,
    InitiatorId,
    Status,
    Reason,
    Resolution,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum DisputeEvidence {
    Table,
    Id,
    DisputeId,
    FileUrl,
    Note,
    CreatedAt,
}

#[derive(Iden)]
enum Merchants {
    Table,
    Id,
    Name,
    LogoUrl,
    BrandColor,
    ContactUserId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum MerchantLocations {
    Table,
    Id,
    MerchantId,
    Name,
    Address,
    CityId,
}

#[derive(Iden)]
enum MerchantUsers {
    Table,
    Id,
    MerchantId,
    UserId,
    Role,
}

#[derive(Iden)]
enum MerchantTemplates {
    Table,
    Id,
    MerchantId,
    Name,
    Description,
    DeliveryRequirements,
    CreatedAt,
}

#[derive(Iden)]
enum MerchantTemplateItems {
    Table,
    Id,
    TemplateId,
    Name,
    Quantity,
    Price,
}

#[derive(Iden)]
enum MerchantContracts {
    Table,
    Id,
    OrderId,
    Terms,
    Version,
    CreatedAt,
}

#[derive(Iden)]
enum MerchantInvoices {
    Table,
    Id,
    MerchantId,
    OrderId,
    Title,
    TaxNo,
    Amount,
    Status,
    CreatedAt,
}

#[derive(Iden)]
enum MerchantApprovals {
    Table,
    Id,
    DemandId,
    MerchantId,
    Status,
    ApproverId,
    Comment,
    CreatedAt,
}

#[derive(Iden)]
enum Conversations {
    Table,
    Id,
    Type,
    OrderId,
    CreatedAt,
}

#[derive(Iden)]
enum Messages {
    Table,
    Id,
    ConversationId,
    SenderId,
    Content,
    MsgType,
    SentAt,
}

#[derive(Iden)]
enum Notifications {
    Table,
    Id,
    UserId,
    Type,
    Title,
    Content,
    ReadAt,
    CreatedAt,
}

#[derive(Iden)]
enum AuditLogs {
    Table,
    Id,
    AdminId,
    Action,
    TargetType,
    TargetId,
    Detail,
    CreatedAt,
}

#[derive(Iden)]
enum Configs {
    Table,
    Id,
    Key,
    Value,
}
