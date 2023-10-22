use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};

pub enum WidgetKind {
    Text(TextWidget),
    Container(ContainerWidget),
}

pub struct WidgetTree {
    root: WidgetKind,
    children: Vec<WidgetTree>,
}

impl WidgetTree {
    pub fn from_vnode(node: &VNode) -> Self {
        let root = match &node.tag {
            NodeKind::Text(props) => {
                let mut text = TextWidget {
                    text: "Hello, world!".to_string(),
                    position: (0.0, 0.0),
                };
                text.patch_props(props);
                WidgetKind::Text(text)
            }
            NodeKind::Container => WidgetKind::Container(ContainerWidget {}),
            NodeKind::Component(comp) => {
                let vnode = (comp)();
                let widget = WidgetTree::from_vnode(&vnode);
                return widget;
            }
        };
        let children = node
            .children
            .iter()
            .map(|child| WidgetTree::from_vnode(child))
            .collect();
        Self { root, children }
    }

    pub fn draw(&self, piet_context: &mut impl RenderContext) {
        match &self.root {
            WidgetKind::Text(widget) => {
                widget.draw(piet_context);
            }
            WidgetKind::Container(widget) => {
                widget.draw(piet_context);
            }
        }
        for child in &self.children {
            child.draw(piet_context);
        }
    }
}

pub struct TextProps {
    pub position: Option<(f64, f64)>,
    pub text: Option<String>,
}

pub enum NodeKind {
    Text(TextProps),
    Container,
    Component(Box<dyn Fn() -> VNode>),
}

impl TextWidget {
    fn patch_props(&mut self, props: &TextProps) {
        if let Some(position) = props.position {
            self.position = position;
        }
        if let Some(text) = &props.text {
            self.text = text.clone();
        }
    }
}

struct ContainerWidget {}

impl Widget for ContainerWidget {
    fn draw(&self, _piet_context: &mut impl RenderContext) {}
}

pub struct VNode {
    pub tag: NodeKind,
    pub children: Vec<VNode>,
}

trait Widget {
    fn draw(&self, piet_context: &mut impl RenderContext);
}

struct TextWidget {
    text: String,
    position: (f64, f64),
}

impl Widget for TextWidget {
    fn draw(&self, piet_context: &mut impl RenderContext) {
        let layout = piet_context
            .text()
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, 12.0)
            .text_color(Color::WHITE)
            .build()
            .unwrap();
        piet_context.draw_text(&layout, self.position);
    }
}
