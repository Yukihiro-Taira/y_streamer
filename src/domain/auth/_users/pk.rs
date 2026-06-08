use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserPk(pub Uuid);

impl From<Uuid> for UserPk {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserPk> for Uuid {
    fn from(pk: UserPk) -> Self {
        pk.0
    }
}

impl AsRef<Uuid> for UserPk {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserPk {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for UserPk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
