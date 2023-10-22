mod elements;
use elements::{NodeKind, TextProps, VNode, WidgetKind, WidgetTree};
use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App {
    comp: Box<dyn Fn() -> VNode>,
    old_vnode: Option<VNode>,
    widget_tree: Option<WidgetTree>,
}

impl App {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        let new_vnode = (self.comp)();
        let tree =
            WidgetTree::diff_root(self.widget_tree.take(), self.old_vnode.as_ref(), &new_vnode);
        self.old_vnode = Some(new_vnode);
        tree.draw(piet_context);
        self.widget_tree = Some(tree);
        piet_context.finish().unwrap();
    }
}

pub fn create_new_app() -> App {
    App {
        comp: Box::new(|| TestComp::new("Hi there, people".to_string()).run()),
        old_vnode: None,
        widget_tree: None,
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
                    tag: NodeKind::Text,
                    children: vec![],
                },
                VNode {
                    tag: NodeKind::Text,
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
                tag: NodeKind::Text,
                children: vec![],
            }],
        }
    }
}
