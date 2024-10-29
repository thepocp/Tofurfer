use crate::css::cascade_context::CascadeContext;
use crate::css::inline_styles::parse_inline_styles;
use crate::css::rules_applier::apply_cascade_styles;
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
                <p id="hello">Hello</p>
                <div class="WORLD">world!</div>
            </body>
        </html>"#;

    let mut dom = parse_html(html);
    let css_rules = parse_inline_styles(&dom);
    let cascade_context = CascadeContext::new(css_rules);
    apply_cascade_styles(&mut dom, &cascade_context);

    dbg!("{:?}", dom);
}
