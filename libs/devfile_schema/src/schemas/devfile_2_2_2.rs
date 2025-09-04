use typify::import_types;

//use serde::{Deserialize, Serialize};
import_types!(schema = "src/schemas/devfile.2.2.2.json");
pub const JSON_TYPE_2_2_2: &str = include_str!("devfile.2.2.2.json");
