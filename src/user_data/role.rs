use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Role {
    Admin,
    Customer,
}
