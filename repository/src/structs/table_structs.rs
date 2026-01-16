use serde::Serialize;

#[derive(Serialize)]
pub struct EntityInfo {
    pub table_name: String,
    pub struct_name: String,
    pub primary_key: PrimaryKeyInfo,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Serialize)]
pub struct PrimaryKeyInfo {
    pub name: String,
    pub rust_type: String,
}

#[derive(Serialize)]
pub struct ColumnInfo {
    pub name: String,
    pub rust_type: String,
}