use crate::css::inline_styles::parse_inline_styles;
use crate::html::parser::parse_html;

mod css;
mod html;

fn main() {
    let html = r#"
        <html>
            <head>
                <title>Title</title>
                <style>
                    p {
                        padding: 10px;
                    }
                    div {
                        margin: 20px;
                    }
                </style>
            </head>
            <body>
                <p id=\"hello\">Hello</p>
                <div class=\"WORLD\">world!</div>
            </body>
        </html>"#;
    let dom = parse_html(&html);
    let styles = parse_inline_styles(&dom);

    println!("{:?}", dom);
    println!("{:?}", styles);
}
