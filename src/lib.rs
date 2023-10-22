use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App {
    comp: Box<dyn Component>,
    root: WidgetTree,
}

enum WidgetKind {
    Text(TextWidget),
    Container(ContainerWidget),
}

struct WidgetTree {
    root: WidgetKind,
    children: Vec<WidgetTree>,
}

impl WidgetTree {
    fn from_vnode(node: &VNode) -> Self {
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
                let vnode = comp.run();
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
}

impl App {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        let vnode = self.comp.run();
        let widget = WidgetTree::from_vnode(&vnode);
        self.root.children = vec![widget];
        self.draw_widget_tree(piet_context, &self.root);
        piet_context.finish().unwrap();
    }

    fn draw_widget_tree(&self, piet_context: &mut impl RenderContext, widget: &WidgetTree) {
        match &widget.root {
            WidgetKind::Text(widget) => {
                widget.draw(piet_context);
            }
            WidgetKind::Container(widget) => {
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
        comp: Box::new(TestComp::new("Hi there, people".to_string())),
        root: WidgetTree {
            root: WidgetKind::Container(ContainerWidget {}),
            children: vec![],
        },
    }
}

struct TextProps {
    position: Option<(f64, f64)>,
    text: Option<String>,
}
enum NodeKind {
    Text(TextProps),
    Container,
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

pub trait Component {
    fn run(&self) -> VNode;
    fn is_dirty(&self) -> bool {
        true
    }
}

struct TestComp {
    text: String,
}

impl TestComp {
    fn new(text: String) -> Self {
        Self { text }
    }
}

impl Component for TestComp {
    fn run(&self) -> VNode {
        VNode {
            tag: NodeKind::Container,
            children: vec![
                VNode {
                    tag: NodeKind::Text(TextProps {
                        position: Some((100., 50.)),
                        text: Some(self.text.clone()),
                    }),
                    children: vec![],
                },
                VNode {
                    tag: NodeKind::Text(TextProps {
                        position: Some((50., 100.)),
                        text: Some(self.text.clone()),
                    }),
                    children: vec![],
                },
                VNode {
                    tag: NodeKind::Component(Box::new(TestComp2::new())),
                    children: vec![],
                },
            ],
        }
    }
}

struct TestComp2 {}

impl TestComp2 {
    fn new() -> Self {
        Self {}
    }
}

impl Component for TestComp2 {
    fn run(&self) -> VNode {
        VNode {
            tag: NodeKind::Container,
            children: vec![VNode {
                tag: NodeKind::Text(TextProps {
                    position: Some((100., 200.)),
                    text: Some("This is Comp 2".to_string()),
                }),
                children: vec![],
            }],
        }
    }
}
