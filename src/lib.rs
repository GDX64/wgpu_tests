use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App {
    comp: Box<dyn Component>,
}

enum WidgetKind {
    Text(TextWidget),
}

struct WidgetTree {
    root: WidgetKind,
    children: Vec<WidgetTree>,
}

impl WidgetTree {
    fn from_vnode(node: &VNode) -> Self {
        let root = match &node.tag {
            NodeKind::Text => {
                WidgetKind::Text(TextWidget {
                    text: "Hello, world!".to_string(),
                })
            }
            NodeKind::Rect => todo!(),
            NodeKind::Component(comp) => todo!(),
        };
        let children = node
            .children
            .iter()
            .map(|child| WidgetTree::from_vnode(child))
            .collect();
        Self { root, children }
    }
}

impl App {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        let vnode = self.comp.run();
        let widget = WidgetTree::from_vnode(&vnode);
        self.draw_widget_tree(piet_context, &widget);
        piet_context.finish().unwrap();
    }

    fn draw_widget_tree(&self, piet_context: &mut impl RenderContext, widget: &WidgetTree) {
        match &widget.root {
            WidgetKind::Text(widget) => {
                widget.draw(piet_context);
            }
        }
        for child in &widget.children {
            self.draw_widget_tree(piet_context, child);
        }
    }
}

pub fn create_new_app() -> App {
    App {
        comp: Box::new(TextComp::new("Hello, world!".to_string())),
    }
}

enum NodeKind {
    Text,
    Rect,
    Component(Box<dyn Component>),
}

pub struct VNode {
    tag: NodeKind,
    children: Vec<VNode>,
}

trait Widget {
    fn draw(&self, piet_context: &mut impl RenderContext);
}

struct TextWidget {
    text: String,
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
        piet_context.draw_text(&layout, (0.0, 0.0));
    }
}

pub trait Component {
    fn run(&mut self) -> VNode;
}

struct TextComp {
    text: String,
}

impl TextComp {
    fn new(text: String) -> Self {
        Self { text }
    }
}

impl Component for TextComp {
    fn run(&mut self) -> VNode {
        VNode {
            tag: NodeKind::Text,
            children: vec![
                VNode {
                    tag: NodeKind::Text,
                    children: vec![],
                },
                VNode {
                    tag: NodeKind::Text,
                    children: vec![],
                },
            ],
        }
    }
}
