use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id{
    String: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordId {
    pub tb: String,
    pub id: Id,
}