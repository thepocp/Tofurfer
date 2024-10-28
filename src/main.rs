use crate::html::parser::parse_html;

mod html;

fn main() {
    let html =
        "<html><head><title>Title</title></head><body><p id=\"hello\">Hello</p><div class=\"WORLD\">world!</div></body></html>";
    let dom = parse_html(&html);
    println!("{:?}", dom);
}
