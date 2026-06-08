use std::sync::Arc;

use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::role::Role;
use crate::domain::auth::app_error::AppError;

pub trait Permission: core::fmt::Debug + Send + Sync {
    fn has_permission(&self, user: &User) -> bool {
        user.has_any_role(&self.roles_required())
    }

    fn check_permission(&self, user: &User) -> Result<(), AppError> {
        if !self.has_permission(user) {
            Err(AppError::Unauthorized)
        } else {
            Ok(())
        }
    }

    fn roles_required(&self) -> Vec<Role>;
}

#[derive(Clone)]
pub enum ConstantPermissions {
    AlwaysAllowed,
    Computed(Arc<dyn Fn(&User) -> bool + Send + Sync>),
}

impl core::fmt::Debug for ConstantPermissions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlwaysAllowed => write!(f, "AlwaysAllowed"),
            Self::Computed(_) => write!(f, "Computed(<closure>)"),
        }
    }
}

impl Permission for ConstantPermissions {
    fn has_permission(&self, user: &User) -> bool {
        match self {
            Self::AlwaysAllowed => true,
            Self::Computed(closure) => closure(user),
        }
    }

    fn roles_required(&self) -> Vec<Role> {
        vec![]
    }
}
