use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Default, Deserialize)]
pub enum LoginState {
    #[default]
    WrongPassword,
    Accepted,
    RepeatLogin,
}
