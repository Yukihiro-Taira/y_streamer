pub struct UsersDb;

#[cfg(feature = "server")]
mod db {
    use std::collections::HashSet;

    use serde::{Deserialize, Serialize};
    use sqlx::{PgPool, query_as};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::domain::auth::_users::data::user::User;
    use crate::domain::auth::_users::pk::UserPk;
    use crate::domain::auth::_users::role::{Role, RoleAccess};

    #[derive(Debug, sqlx::FromRow)]
    struct SqlUserBasic {
        unid: Uuid,
        firstname: String,
        lastname: String,
        email: String,
        password: String,
    }

    #[derive(Debug, sqlx::FromRow)]
    struct SqlRoleAccess {
        role: Role,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
    pub struct UserLoginDto {
        pub unid: Uuid,
        pub email: String,
        pub password: String,
    }

    impl UsersDb {
        /// Load full user by UUID. Used by `Authentication::load_user`.
        pub async fn get_by_unid(pool: &PgPool, unid: Uuid) -> Result<Option<User>, sqlx::Error> {
            let sql_user = query_as!(
                SqlUserBasic,
                r#"
                    SELECT unid, firstname, lastname, email, password
                    FROM app_schema.users
                    WHERE unid = $1
                "#,
                unid
            )
            .fetch_optional(pool)
            .await?;

            let Some(sql_user) = sql_user else {
                return Ok(None);
            };

            let sql_roles = query_as!(
                SqlRoleAccess,
                r#"
                    SELECT role as "role: Role"
                    FROM app_schema.roleaccesses
                    WHERE grantedto_unid = $1
                "#,
                sql_user.unid
            )
            .fetch_all(pool)
            .await?;

            let roles: HashSet<RoleAccess> = sql_roles
                .into_iter()
                .map(|r| {
                    tracing::debug!("Loaded role {:?} for user {}", r.role, sql_user.unid);
                    RoleAccess { role: r.role }
                })
                .collect();

            Ok(Some(User {
                unid: UserPk::from(sql_user.unid),
                created: OffsetDateTime::UNIX_EPOCH,
                first_name: sql_user.firstname,
                last_name: sql_user.lastname,
                last_password_change: OffsetDateTime::UNIX_EPOCH,
                email: sql_user.email,
                password: sql_user.password,
                roles,
            }))
        }

        /// All users ordered by name.
        pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
            let sql_users = query_as!(
                SqlUserBasic,
                r#"
                    SELECT unid, firstname, lastname, email, password
                    FROM app_schema.users
                    ORDER BY firstname, lastname
                "#
            )
            .fetch_all(pool)
            .await?;

            let mut users = Vec::new();
            for sql_user in sql_users {
                let sql_roles = query_as!(
                    SqlRoleAccess,
                    r#"
                        SELECT role as "role: Role"
                        FROM app_schema.roleaccesses
                        WHERE grantedto_unid = $1
                    "#,
                    sql_user.unid
                )
                .fetch_all(pool)
                .await?;

                let roles: HashSet<RoleAccess> = sql_roles
                    .into_iter()
                    .map(|r| RoleAccess { role: r.role })
                    .collect();

                users.push(User {
                    unid: UserPk::from(sql_user.unid),
                    created: OffsetDateTime::UNIX_EPOCH,
                    first_name: sql_user.firstname,
                    last_name: sql_user.lastname,
                    last_password_change: OffsetDateTime::UNIX_EPOCH,
                    email: sql_user.email,
                    password: sql_user.password,
                    roles,
                });
            }
            Ok(users)
        }

        /// Minimal fields for login verification.
        pub async fn get_from_email(email: String, pool: &PgPool) -> Option<UserLoginDto> {
            query_as!(
                UserLoginDto,
                r#"
                    SELECT unid, email, password
                    FROM app_schema.users
                    WHERE email = $1
                "#,
                email
            )
            .fetch_one(pool)
            .await
            .ok()
        }
    }
}
