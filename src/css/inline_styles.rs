use crate::css::models::StyleRule;
use crate::css::parser::parse_css;
use crate::html::models::Node;

fn extract_all_styles(node: &Node, styles: &mut String) {
    match node {
        Node::Element { tag, children, .. } if tag == "style" => {
            for child in children {
                if let Node::Text(content) = child {
                    styles.push_str(content);
                    styles.push('\n');
                }
            }
        }
        Node::Element { children, .. } => {
            for child in children {
                extract_all_styles(child, styles);
            }
        }
        _ => {}
    }
}

pub fn parse_inline_styles(root: &Node) -> Vec<StyleRule> {
    let mut all_styles = String::new();
    extract_all_styles(root, &mut all_styles);
    parse_css(&all_styles)
}
