use std::fs;
use tera::{Context, Tera};
use crate::structs::table_structs::EntityInfo;
use crate::helpers::parser_converter::to_pascal_case;

pub fn generate_mod_file(repos: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut mod_content = String::from("#[allow(unused_imports)]\n// Auto-generated module file\n\n");

    for repo in repos {
        mod_content.push_str(&format!("pub mod {}_repository;\n", repo));
    }

    mod_content.push_str("\n// Re-export for convenience\n");
    for repo in repos {
        let struct_name = to_pascal_case(repo);
        mod_content.push_str(&format!(
            "\n#[allow(unused_imports)]\npub use {}_repository::{}Repository;\n",
            repo, struct_name
        ));
    }

    let mod_path = "./hub-be-repository/src/repositories/mod.rs";
    fs::write(mod_path, mod_content)?;
    println!("Generated: {}", mod_path);

    Ok(())
}

pub fn generate_repository(
    tera: &Tera,
    entity: &EntityInfo
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("table_name", &entity.table_name);
    context.insert("entity_name", &entity.struct_name);
    context.insert("primary_key", &entity.primary_key);
    context.insert("columns", &entity.columns);

    let output = tera.render("repository.tera", &context)?;

    let filename = format!("./hub-be-repository/src/repositories/{}_repository.rs", entity.table_name);
    fs::write(&filename, output)?;

    println!("Generated: {}", filename);
    Ok(())
}