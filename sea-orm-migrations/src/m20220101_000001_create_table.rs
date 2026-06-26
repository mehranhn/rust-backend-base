use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table("users")
					.if_not_exists()
					.col(uuid("id").primary_key())
					.col(
						date_time("created_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(
						date_time("updated_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(integer("role"))
					.col(string("username").string_len(255))
					.col(binary("hashed_password"))
					.col(string("phone").string_len(21).null())
					.col(string("email").string_len(255).null())
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_users_username")
					.table("users")
					.col("username")
					.unique()
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table("sessions")
					.if_not_exists()
					.col(uuid("id").primary_key())
					.col(
						date_time("created_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(
						date_time("updated_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(uuid("user_id"))
					.col(date_time("expire_at").timestamp_with_time_zone().null())
					.col(
						date_time("last_access")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(date_time("deleted_at").timestamp_with_time_zone().null())
					.foreign_key(
						ForeignKey::create()
							.name("fk_session_user")
							.from("sessions", "user_id")
							.to("users", "id")
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_sessions_user_id")
					.table("sessions")
					.col("user_id")
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table("posts")
					.if_not_exists()
					.col(uuid("id").primary_key())
					.col(
						date_time("created_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(
						date_time("updated_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(uuid("user_id"))
					.col(string("content").text())
					.foreign_key(
						ForeignKey::create()
							.name("fk_post_user")
							.from("posts", "user_id")
							.to("users", "id")
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_posts_user_id")
					.table("posts")
					.col("user_id")
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table("comments")
					.if_not_exists()
					.col(uuid("id").primary_key())
					.col(
						date_time("created_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(
						date_time("updated_at")
							.timestamp_with_time_zone()
							.default(Expr::current_timestamp()),
					)
					.col(uuid("user_id"))
					.col(uuid("post_id"))
					.col(uuid("reply_to_id"))
					.col(string("content").text())
					.foreign_key(
						ForeignKey::create()
							.name("fk_comment_user")
							.from("comments", "user_id")
							.to("users", "id")
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_comment_post")
							.from("comments", "post_id")
							.to("posts", "id")
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_comment_replies")
							.from("comments", "reply_to_id")
							.to("comments", "id")
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_comments_user_id")
					.table("comments")
					.col("user_id")
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_comments_post_id")
					.table("comments")
					.col("post_id")
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.name("idx_comments_reply_to_id")
					.table("comments")
					.col("reply_to_id")
					.to_owned(),
			)
			.await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table("comments").to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table("posts").to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table("sessions").to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table("users").to_owned())
			.await?;
		Ok(())
	}
}
