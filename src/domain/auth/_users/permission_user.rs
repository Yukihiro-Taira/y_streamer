use serde::{Deserialize, Serialize};

use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::role::Role;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserPermission {
    ListAll,
}

impl Permission for UserPermission {
    fn roles_required(&self) -> Vec<Role> {
        match self {
            Self::ListAll => vec![Role::Root],
        }
    }
}
