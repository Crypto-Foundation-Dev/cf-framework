use ammonia::Builder;
use maplit::{hashmap, hashset};

pub fn sanitize_content_html_tiptap(html: &str) -> String {
    Builder::default()
        .add_tags(&["mark", "code", "pre", "figure", "figcaption"])

        .add_tag_attributes("p", &["data-type"])
        .add_tag_attributes("span", &["style", "class"])
        .add_tag_attributes("img", &["title", "width", "height", "alt"])
        .allowed_classes(hashmap![
        "code" => hashset!["language-rust", "language-js", "language-python"],
        "pre" => hashset!["language-rust", "language-js", "language-python"],
        "p" => hashset!["text-left", "text-center", "text-right"]
        ])

        .set_tag_attribute_value("a", "target", "_blank")

        .clean(html)
        .to_string()
}