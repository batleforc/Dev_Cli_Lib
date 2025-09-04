use typify::import_types;

//use serde::{Deserialize, Serialize};
import_types!(schema = "src/schemas/devfile.2.3.0.json");
pub const JSON_TYPE_2_3_0: &str = include_str!("devfile.2.3.0.json");
