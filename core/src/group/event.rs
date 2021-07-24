use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

// #[derive(Serialize, Deserialize)]
// pub struct Metadata {
//     pub user_id: String,
// }

#[derive(Debug, Serialize, PartialEq, EnumString, AsRefStr, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum Event {
    Created,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Created {
    pub user_id: String,
    pub name: String,
}
