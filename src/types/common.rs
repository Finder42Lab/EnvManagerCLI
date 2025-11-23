use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IIdName {
    pub id: String,
    pub name: String,

}