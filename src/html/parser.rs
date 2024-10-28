use crate::html::models::Node;
use std::collections::HashMap;

pub fn parse_html(input: &str) -> Node {
    let mut chars = input.chars().peekable();
    parse_element(&mut chars)
}

fn parse_element<I>(chars: &mut std::iter::Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut children = Vec::new();
    let (tag, attributes) = parse_tag(chars);

    while let Some(&ch) = chars.peek() {
        if ch == '<' {
            chars.next();
            if let Some('/') = chars.peek() {
                while chars.next() != Some('>') {}
                break;
            } else {
                children.push(parse_element(chars));
            }
        } else {
            let text = parse_text(chars);
            if !text.is_empty() {
                children.push(Node::Text(text));
            }
        }
    }

    Node::Element {
        tag,
        children,
        attributes,
    }
}

fn parse_tag<I>(chars: &mut std::iter::Peekable<I>) -> (String, HashMap<String, String>)
where
    I: Iterator<Item = char>,
{
    let mut tag = String::new();
    let mut attributes = HashMap::new();

    if chars.peek() == Some(&'<') {
        chars.next();
    }


    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() || ch == '>' {
            break;
        } else {
            tag.push(ch);
            chars.next();
        }
    }

    while let Some(&ch) = chars.peek() {
        if ch == '>' {
            chars.next();
            break;
        } else if ch.is_whitespace() {
            chars.next();
            if let Some((attr_name, attr_value)) = parse_attribute(chars) {
                attributes.insert(attr_name, attr_value);
            }
        } else {
            chars.next();
        }
    }

    (tag, attributes)
}

fn parse_attribute<I>(chars: &mut std::iter::Peekable<I>) -> Option<(String, String)>
where
    I: Iterator<Item = char>,
{
    let mut name = String::new();
    let mut value = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '=' {
            chars.next();
            break;
        } else if ch.is_whitespace() || ch == '>' {
            return None;
        } else {
            name.push(ch);
            chars.next();
        }
    }

    if chars.peek() == Some(&'"') {
        chars.next();
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next();
                break;
            } else {
                value.push(ch);
                chars.next();
            }
        }
    }

    Some((name, value))
}

fn parse_text<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut text = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '<' {
            break;
        } else {
            text.push(ch);
            chars.next();
        }
    }
    text.trim().to_string()
}
