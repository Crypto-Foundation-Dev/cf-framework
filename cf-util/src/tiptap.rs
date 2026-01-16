use serde_json::Value;

fn extract_text_from_tiptap(json: &Value) -> String {
    let mut text_parts = Vec::new();

    fn traverse(node: &Value, text_parts: &mut Vec<String>) {
        // Jika node punya text field, ambil
        if let Some(text) = node.get("text").and_then(|v| v.as_str()) {
            text_parts.push(text.to_string());
        }

        // Traverse content array jika ada
        if let Some(content) = node.get("content").and_then(|v| v.as_array()) {
            for child in content {
                traverse(child, text_parts);
            }
        }
    }

    traverse(json, &mut text_parts);
    text_parts.join(" ")
}

pub fn create_excerpt(json: &Value, word_limit: usize) -> String {
    let full_text = extract_text_from_tiptap(json);
    let words: Vec<&str> = full_text.split_whitespace().collect();

    if words.len() <= word_limit {
        return full_text;
    }

    let excerpt = words[..word_limit].join(" ");
    format!("{}...", excerpt)
}