use crate::css::models::StyleRule;
use std::collections::HashMap;

pub fn parse_css(css: &str) -> Vec<StyleRule> {
    let mut rules = Vec::new();

    for rule in css.split('}') {
        if let Some((selector, properties)) = rule.split_once('{') {
            let mut prop_map = HashMap::new();
            for property in properties.trim().split(';') {
                if let Some((key, value)) = property.split_once(':') {
                    prop_map.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            rules.push(StyleRule {
                selector: selector.trim().to_string(),
                properties: prop_map,
            });
        }
    }

    rules
}
