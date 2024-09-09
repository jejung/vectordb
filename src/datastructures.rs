use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) const _FLAG_STORE_ONLY: u32 = 0;
pub(super) const _FLAG_SEARCHABLE: u32 = 1;
pub(super) const _FLAG_FILTERABLE: u32 = 1 << 1;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Field {
    name: String,
    values: Vec<String>,
    flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    id: Option<String>,
    fields: Vec<Field>,
    vectors: HashMap<String, Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct OperationError {
    code: String,
    message: String,
}

pub(super) struct _InsertOperation {
    documents: Vec<Result<Document, OperationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct InsertResult {
    id: String,
    success: bool,
    errors: Option<Vec<OperationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct InsertResponse {
    inserts: Vec<InsertResult>
}
