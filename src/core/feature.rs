use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub enum FeaturePlan<F> {
    Feature(F, Box<FeaturePlan<F>>),
    Branch(Vec<FeaturePlan<F>>),
}

impl<F> FeaturePlan<F> {
    pub fn new() -> Self {
        FeaturePlan::Branch(Vec::new())
    }

    pub fn new_feature(feature: F) -> Self {
        FeaturePlan::new().prepended(feature)
    }

    pub fn prepended(self, feature: F) -> Self {
        FeaturePlan::Feature(feature, Box::new(self))
    }

    pub fn find_feature_depth<'a, P>(&'a self, predicate: &P) -> Option<usize>
    where
        P: Fn(&'a F) -> bool,
        F: 'a,
    {
        match self {
            FeaturePlan::Feature(feature, next) => {
                if predicate(feature) {
                    Some(0)
                } else {
                    next.find_feature_depth(predicate).map(|depth| depth + 1)
                }
            }
            FeaturePlan::Branch(nodes) => nodes
                .iter()
                .map(|node| node.find_feature_depth(predicate).map(|depth| depth + 1))
                .fold(None, |acc, depth| match (acc, depth) {
                    (Some(acc), Some(depth)) => Some(acc.min(depth)),
                    (acc, depth) => acc.or(depth),
                }),
        }
    }

    pub fn max_depth(&self) -> usize {
        match self {
            FeaturePlan::Feature(_, next) => next.max_depth() + 1,
            FeaturePlan::Branch(nodes) => nodes
                .iter()
                .fold(1, |acc, node| acc.max(node.max_depth() + 1)),
        }
    }
}

impl<F> FeaturePlan<F>
where
    F: Clone,
{
    pub fn join(&mut self, other: Self) {
        match (self, other) {
            (FeaturePlan::Branch(ref mut u), FeaturePlan::Branch(mut v)) => {
                u.append(&mut v);
            }
            (FeaturePlan::Branch(ref mut u), feature) => {
                u.push(feature);
            }
            (this, FeaturePlan::Branch(mut u)) => {
                u.push((*this).clone());
                *this = FeaturePlan::Branch(u);
            }
            (this, f) => {
                *this = FeaturePlan::Branch(vec![(*this).clone(), f]);
            }
        }
    }

    pub fn prepend(&mut self, feature: F) {
        *self = FeaturePlan::Feature(feature, Box::new(self.clone()));
    }

    pub fn skip_feature(&mut self) {
        if let FeaturePlan::Feature(_, next) = self {
            *self = (**next).clone();
        }
    }
}

impl<F> FeaturePlan<F>
where
    F: Feature,
{
    pub fn into_room(self) -> F::Room {
        match self {
            FeaturePlan::Feature(feature, child) => {
                let (Ok(room) | Err(room)) =
                    feature.apply(child.into_room()).map_err(|(feature, room)| {
                        feature
                            .apply(F::Room::default().add_exits(vec![room]))
                            .ok()
                            .unwrap()
                    });
                room
            }
            FeaturePlan::Branch(nodes) => {
                let nodes: Vec<_> = nodes.into_iter().map(|node| node.into_room()).collect();
                F::Room::default().add_exits(nodes)
            }
        }
    }
}

impl<F> FeaturePlan<F>
where
    F: Debug,
{
    pub fn show(&self) {
        fn visit<F>(node: &FeaturePlan<F>, mark: bool, indent: usize)
        where
            F: Debug,
        {
            let prefix = if mark { "+ " } else { "  " };
            match node {
                FeaturePlan::Feature(e, t) => {
                    println!("{:indent$}{}{:?}", "", prefix, e, indent = indent);
                    visit(t, false, indent);
                }
                FeaturePlan::Branch(ts) => {
                    if !ts.is_empty() {
                        println!("{:indent$}{}<branch>", "", prefix, indent = indent);
                        for t in ts {
                            visit(t, true, indent + 2);
                        }
                    }
                }
            }
        }
        visit(self, true, 0);
    }
}

impl<F> Default for FeaturePlan<F>
where
    F: Feature + Debug,
{
    fn default() -> Self {
        FeaturePlan::new()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Command<F>
where
    F: Debug,
{
    New(F),
    PrependOne(F),
    PrependEach(F),
    PrependGrouped(F),
}

impl<F> FeaturePlan<F>
where
    F: Debug + Eq + Feature + Hash,
{
    pub fn from_steps<J, P, I>(
        mut join_selector: J,
        mut prepend_selector: P,
        steps: I,
    ) -> FeaturePlan<F>
    where
        J: FnMut(&[FeaturePlan<F>]) -> Option<(usize, usize)>,
        P: FnMut(&[FeaturePlan<F>]) -> usize,
        I: IntoIterator<Item = Command<F>>,
    {
        let mut feature_plan = FeaturePlan::new();

        for step in steps {
            feature_plan = match step {
                Command::New(feature) => {
                    let feature = FeaturePlan::new_feature(feature);
                    let mut nodes = match feature_plan {
                        FeaturePlan::Branch(mut nodes) => {
                            nodes.push(feature);
                            nodes
                        }
                        _ => vec![feature_plan, feature],
                    };
                    if nodes.len() > 2 {
                        if let Some((i, j)) = join_selector(&nodes) {
                            let mut join_nodes = Vec::new();
                            if i < j {
                                join_nodes.push(nodes.swap_remove(j));
                                join_nodes.push(nodes.swap_remove(i));
                            } else {
                                join_nodes.push(nodes.swap_remove(i));
                                join_nodes.push(nodes.swap_remove(j));
                            }
                            nodes.push(FeaturePlan::Branch(join_nodes));
                        }
                    }
                    FeaturePlan::Branch(nodes)
                }
                Command::PrependOne(feature) => match feature_plan {
                    FeaturePlan::Branch(mut nodes) => {
                        let i = prepend_selector(&nodes);
                        nodes.get_mut(i).unwrap().prepend(feature);
                        FeaturePlan::Branch(nodes)
                    }
                    _ => feature_plan.prepended(feature),
                },
                Command::PrependEach(feature) => match feature_plan {
                    FeaturePlan::Branch(nodes) => FeaturePlan::Branch(
                        nodes
                            .into_iter()
                            .map(|node| node.prepended(feature))
                            .collect(),
                    ),
                    _ => feature_plan.prepended(feature),
                },
                Command::PrependGrouped(feature) => feature_plan.prepended(feature),
            }
        }
        feature_plan
    }
}

pub trait Feature: Copy + Debug {
    type Room: Default + Room;

    fn apply(self, room: Self::Room) -> Result<Self::Room, (Self, Self::Room)>;
}

pub trait Room: Sized {
    fn add_exits<I>(self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn leaf() -> FeaturePlan<()> {
        FeaturePlan::Branch(Vec::new())
    }

    pub fn feature(feature_plan: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Feature((), Box::new(feature_plan))
    }

    pub fn branch1(node: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Branch(vec![node])
    }

    pub fn branch2(node1: FeaturePlan<()>, node2: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Branch(vec![node1, node2])
    }

    #[test]
    fn test_max_depth() {
        assert_eq!(1, leaf().max_depth());
        assert_eq!(2, branch1(leaf()).max_depth());
        assert_eq!(2, branch2(leaf(), leaf()).max_depth());
        assert_eq!(3, branch1(branch1(leaf())).max_depth());
        assert_eq!(2, feature(leaf()).max_depth());
        assert_eq!(3, feature(feature(leaf())).max_depth());

        assert_eq!(
            7,
            branch2(
                feature(leaf()),
                feature(branch2(
                    feature(feature(feature(leaf()))),
                    feature(feature(feature(leaf()))),
                )),
            )
            .max_depth()
        );
    }
}
