use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};

pub enum WidgetKind {
    Text(TextWidget),
    Container(ContainerWidget),
}

pub struct WidgetTree {
    node: WidgetKind,
    children: Vec<WidgetTree>,
}

impl WidgetTree {
    pub fn from_vnode(node: &VNode) -> Self {
        println!("from_vnode: ");
        let root = match &node.tag {
            NodeKind::Text => {
                let text = TextWidget {
                    text: "Hello, world!".to_string(),
                    position: (0.0, 0.0),
                };
                WidgetKind::Text(text)
            }
            NodeKind::Container => WidgetKind::Container(ContainerWidget {}),
        };
        let children = node
            .children
            .iter()
            .map(|child| WidgetTree::from_vnode(child))
            .collect();
        Self {
            node: root,
            children,
        }
    }

    pub fn diff_root(tree: Option<Self>, old_node: Option<&VNode>, new_node: &VNode) -> Self {
        let changes = Self::diff_nodes(old_node, Some(new_node));
        let new_root = Self::reconcile(tree, changes);
        return new_root.unwrap();
    }

    fn reconcile(me: Option<Self>, changes: Changes) -> Option<Self> {
        match changes {
            Changes::None => me,
            Changes::Removed(_) => None,
            Changes::Inserted(new) => Some(Self::from_vnode(new)),
            Changes::Updated(new) => Some(Self::from_vnode(new)),
            Changes::Children(children_changes) => {
                let mut new_children = vec![];
                let Self { children, node } = me.unwrap();
                let mut children = children.into_iter();
                for child in children_changes.into_iter() {
                    let old_child = children.next();
                    let new_child = Self::reconcile(old_child, child);
                    if let Some(new_child) = new_child {
                        new_children.push(new_child);
                    }
                }
                Some(Self {
                    node: node,
                    children: new_children,
                })
            }
        }
    }

    pub fn draw(&self, piet_context: &mut impl RenderContext) {
        match &self.node {
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

    fn diff_nodes<'a>(old: Option<&'a VNode>, new: Option<&'a VNode>) -> Changes<'a> {
        match (old, new) {
            (Some(old), Some(new)) => {
                if old == new {
                    Changes::None
                } else if old.tag != new.tag {
                    Changes::Updated(new)
                } else {
                    let mut children = vec![];
                    let max_size = std::cmp::max(old.children.len(), new.children.len());
                    for i in 0..max_size {
                        let old_child = old.children.get(i);
                        let new_child = new.children.get(i);
                        let diff = Self::diff_nodes(old_child, new_child);
                        children.push(diff);
                    }
                    Changes::Children(children)
                }
            }
            (Some(old), None) => Changes::Removed(old),
            (None, Some(new)) => Changes::Inserted(new),
            (None, None) => Changes::None,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Changes<'a> {
    None,
    Inserted(&'a VNode),
    Removed(&'a VNode),
    Children(Vec<Changes<'a>>),
    Updated(&'a VNode),
}

#[derive(PartialEq)]
pub struct TextProps {
    pub position: Option<(f64, f64)>,
    pub text: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Text,
    Container,
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

#[derive(PartialEq, Debug)]
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

mod test {
    use super::{NodeKind, VNode};

    #[test]
    fn test_diff() {
        let tree1 = VNode {
            children: vec![
                VNode {
                    children: vec![],
                    tag: NodeKind::Text,
                },
                VNode {
                    children: vec![],
                    tag: NodeKind::Text,
                },
            ],
            tag: NodeKind::Container,
        };
        let tree2 = VNode {
            children: vec![
                VNode {
                    children: vec![],
                    tag: NodeKind::Container,
                },
                VNode {
                    children: vec![],
                    tag: NodeKind::Text,
                },
                VNode {
                    children: vec![],
                    tag: NodeKind::Text,
                },
            ],
            tag: NodeKind::Container,
        };
        let diff = super::WidgetTree::diff_nodes(Some(&tree1), Some(&tree2));
        assert_eq!(
            diff,
            super::Changes::Children(vec![
                super::Changes::Updated(&tree2.children[0]),
                super::Changes::None,
                super::Changes::Inserted(&tree2.children[2]),
            ])
        );
    }
}
