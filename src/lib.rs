mod elements;
use elements::{NodeKind, TextProps, VNode, WidgetKind, WidgetTree};
use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App {
    comp: Box<dyn Fn() -> VNode>,
}

impl App {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        let vnode = (self.comp)();
        let widget = WidgetTree::from_vnode(&vnode);
        widget.draw(piet_context);
        piet_context.finish().unwrap();
    }
}

pub fn create_new_app() -> App {
    App {
        comp: Box::new(|| TestComp::new("Hi there, people".to_string()).run()),
    }
}

pub trait Component {
    type Props;
    fn run(&self) -> VNode;
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
    type Props = ();
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
                TestComp2::new().run(),
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
    type Props = ();
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
