use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::pk::UserPk;
use crate::domain::auth::_users::role::{Role, RoleAccess};
use crate::domain::auth::app_error::AppError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct User {
    pub unid: UserPk,
    pub created: OffsetDateTime,
    pub first_name: String,
    pub last_name: String,
    pub last_password_change: OffsetDateTime,
    pub email: String,
    pub password: String,
    pub roles: HashSet<RoleAccess>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            unid: Uuid::new_v4().into(),
            created: OffsetDateTime::UNIX_EPOCH,
            first_name: String::new(),
            last_name: String::new(),
            last_password_change: OffsetDateTime::UNIX_EPOCH,
            email: String::new(),
            password: String::new(),
            roles: HashSet::new(),
        }
    }
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn has_all_roles(&self, roles: &[Role]) -> bool {
        roles.iter().all(|role| {
            self.roles
                .iter()
                .any(|current_role_access| current_role_access.role == *role)
        })
    }

    pub fn has_any_role(&self, roles: &[Role]) -> bool {
        self.roles
            .iter()
            .any(|role_access| roles.contains(&role_access.role))
    }

    pub fn has_permission<P>(&self, permission: &P) -> bool
    where
        P: Permission,
    {
        permission.has_permission(self)
    }

    pub fn has_any_permission<I, P>(&self, permissions: I) -> bool
    where
        I: IntoIterator<Item = P>,
        P: Permission,
    {
        permissions
            .into_iter()
            .any(|permission| permission.has_permission(self))
    }

    pub fn check_permission<P>(&self, permission: &P) -> Result<(), AppError>
    where
        P: Permission,
    {
        permission.check_permission(self)
    }
}
