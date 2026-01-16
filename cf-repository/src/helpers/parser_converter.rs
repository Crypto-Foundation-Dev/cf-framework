use std::fs;
use std::path::Path;
use syn::{File, Item};
use crate::structs::table_structs::{ColumnInfo, EntityInfo, PrimaryKeyInfo};

// Helper function untuk convert snake_case ke PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

pub fn parse_entity_file(path: &Path) -> Result<EntityInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let ast: File = syn::parse_file(&content)?;

    let mut table_name = String::new();
    let mut columns = Vec::new();
    let mut primary_key = None;

    // Parse AST untuk cari struct Model
    for item in ast.items {
        match item {
            Item::Struct(s) if s.ident == "Model" => {

                // Parse fields
                if let syn::Fields::Named(fields) = s.fields {
                    for field in fields.named {
                        let field_name = field.ident.unwrap().to_string();
                        let type_name = extract_type(&field.ty);

                        // Check apakah primary key
                        let is_pk = field.attrs.iter().any(|attr| {
                            if attr.path().is_ident("sea_orm") {
                                if let Ok(meta_list) = attr.meta.require_list() {
                                    return meta_list.tokens.to_string().contains("primary_key");
                                }
                            }
                            false
                        });

                        if is_pk {
                            primary_key = Some(PrimaryKeyInfo {
                                name: field_name.clone(),
                                rust_type: type_name.clone(),
                            });
                        }

                        columns.push(ColumnInfo {
                            name: field_name,
                            rust_type: type_name,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    // Ambil table name dari nama file jika tidak ada di attribute
    if table_name.is_empty() {
        table_name = path.file_stem().unwrap().to_str().unwrap().to_string();
    }

    // Generate struct name dari table name (coupons -> Coupons)
    let struct_name = to_pascal_case(&table_name);

    // Validasi primary key exists
    if primary_key.is_none() {
        return Err(format!("No primary key found in {:?}", path).into());
    }

    Ok(EntityInfo {
        table_name,
        struct_name,
        primary_key: primary_key.unwrap(),
        columns,
    })
}

pub fn extract_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            type_path.path.segments.last()
                .map(|seg| seg.ident.to_string())
                .unwrap_or_default()
        }
        _ => "Unknown".to_string(),
    }
}