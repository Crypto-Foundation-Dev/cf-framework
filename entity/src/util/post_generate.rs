use std::fs;
use std::path::Path;
use regex::Regex;

pub fn post_process_entities(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);

    if !path.is_dir() {
        return Ok(());
    }

    // Fields to add skip_serializing_if
    let fields_to_skip = vec!["deleted_at", "updated_at"];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

            // Skip mod.rs and prelude.rs
            if filename == "mod" || filename == "prelude" {
                continue;
            }

            let content = fs::read_to_string(&path)?;

            // Apply both transformations
            let mut modified = add_serde_skip_attributes(&content, &fields_to_skip);
            modified = add_schema_alias(&modified, filename)?;

            if content != modified {
                fs::write(&path, modified)?;
                println!("  ✓ Modified: {}", path.display());
            }
        }
    }

    Ok(())
}

pub fn add_serde_skip_attributes(content: &str, fields: &[&str]) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        // Check if this line declares one of the fields we want to skip
        let should_add_attribute = fields.iter().any(|field| {
            let pattern = format!(r"pub {}: Option<", field);
            line.trim().starts_with(&pattern)
        });

        if should_add_attribute {
            // Check if the attribute already exists on the previous line
            let has_attribute = i > 0 &&
                lines[i - 1].contains("skip_serializing_if");

            if !has_attribute {
                // Add the attribute with proper indentation
                let indent = line.len() - line.trim_start().len();
                result.push_str(&format!(
                    "{}#[serde(skip_serializing_if = \"Option::is_none\")]\n",
                    " ".repeat(indent)
                ));
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

pub fn add_schema_alias(content: &str, table_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Generate schema name from table name (snake_case to PascalCase)
    let schema_name = snake_to_pascal(table_name);

    // Check if #[schema(as = ...)] already exists
    if content.contains("#[schema(as = ") {
        return Ok(content.to_string());
    }

    // Add #[schema(as = SchemaName)] after #[sea_orm(table_name = "...")]
    let re = Regex::new(r#"(#\[sea_orm\(table_name = "[^"]+"\)\])\s*\n(\s*)(pub struct Model)"#)?;

    let new_content = re.replace(
        content,
        |caps: &regex::Captures| {
            let sea_orm_line = &caps[1];
            let indent = &caps[2];
            let struct_line = &caps[3];

            format!(
                "{}\n{}#[schema(as = {})]\n{}{}",
                sea_orm_line,
                indent,
                schema_name,
                indent,
                struct_line
            )
        }
    );

    Ok(new_content.to_string())
}

fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect()
}
