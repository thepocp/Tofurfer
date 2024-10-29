use crate::css::cascade_context::CascadeContext;
use crate::html::models::Node;

pub fn apply_cascade_styles(node: &mut Node, context: &CascadeContext) {
    if let Node::Element {
        tag,
        styles,
        children,
        ..
    } = node
    {
        let computed_styles = context.compute_styles(tag);
        styles.extend(computed_styles);

        for child in children {
            apply_cascade_styles(child, context);
        }
    }
}
