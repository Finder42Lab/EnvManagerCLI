use serde::{Deserialize, Serialize};
use crate::types::common::{IIdName};
#[derive(Deserialize, Serialize, Debug)]
pub struct IVariable {
    pub id: String,
    pub name: String,
    pub value: Option<String>,
    pub is_secret: bool,
    pub project: IIdName,
}