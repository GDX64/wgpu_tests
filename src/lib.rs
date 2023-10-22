mod elements;
use std::any::Any;

use elements::{NodeKind, TextProps, VNode, WidgetKind, WidgetTree};
use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App<S> {
    comp: Box<dyn Component<State = S>>,
    old_vnode: Option<VNode>,
    widget_tree: Option<WidgetTree>,
    state: S,
}

impl<S> App<S> {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        let new_vnode = self.comp.run(&self.state);
        let tree =
            WidgetTree::diff_root(self.widget_tree.take(), self.old_vnode.as_ref(), &new_vnode);
        self.old_vnode = Some(new_vnode);
        tree.draw(piet_context);
        self.widget_tree = Some(tree);
        piet_context.finish().unwrap();
    }

    pub fn set_state(&mut self, state: S) {
        self.state = state;
    }
}

pub fn create_new_app() -> App<(f32, f32)> {
    App {
        comp: Box::new(TestComp::new("Hi there, people".to_string())),
        old_vnode: None,
        widget_tree: None,
        state: (0., 0.),
    }
}

pub trait Component {
    type State;
    fn run(&self, s: &Self::State) -> VNode;
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
    type State = (f32, f32);
    fn run(&self, s: &Self::State) -> VNode {
        let comps = (s.0 / 10.0) as usize;
        println!("comps: {}", comps);
        VNode {
            tag: NodeKind::Container,
            children: (0..comps)
                .map(|_| {
                    VNode {
                        tag: NodeKind::Text,
                        children: vec![],
                    }
                })
                .collect(),
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
    type State = ();
    fn run(&self, s: &Self::State) -> VNode {
        VNode {
            tag: NodeKind::Container,
            children: vec![VNode {
                tag: NodeKind::Text,
                children: vec![],
            }],
        }
    }
}
