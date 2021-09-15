use rand::Rng;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone)]
pub enum Tree<F>
where
    F: Feature,
{
    Feature(F, Box<Tree<F>>),
    Branch(Vec<Tree<F>>),
}

impl<F> Tree<F>
where
    F: Feature,
{
    pub fn new() -> Self {
        Tree::Branch(Vec::new())
    }

    pub fn new_feature(feature: F) -> Self {
        Tree::new().prepended(feature)
    }

    pub fn join(&mut self, other: Self) {
        match (self, other) {
            (Tree::Branch(ref mut u), Tree::Branch(mut v)) => {
                u.append(&mut v);
            }
            (Tree::Branch(ref mut u), feature) => {
                u.push(feature);
            }
            (this, Tree::Branch(mut u)) => {
                u.push((*this).clone());
                *this = Tree::Branch(u);
            }
            (this, f) => {
                *this = Tree::Branch(vec![(*this).clone(), f]);
            }
        }
    }

    pub fn prepend(&mut self, feature: F) {
        *self = Tree::Feature(feature, Box::new(self.clone()));
    }

    pub fn prepended(self, feature: F) -> Self {
        Tree::Feature(feature, Box::new(self))
    }

    pub fn skip_feature(&mut self) {
        if let Tree::Feature(_, next) = self {
            *self = (**next).clone();
        }
    }

    pub fn find_feature_depth<'a, P>(&'a self, predicate: &P) -> Option<usize>
    where
        P: Fn(&'a F) -> bool,
        F: 'a,
    {
        match self {
            Tree::Feature(feature, next) => {
                if predicate(feature) {
                    Some(0)
                } else {
                    next.find_feature_depth(predicate).map(|depth| depth + 1)
                }
            }
            Tree::Branch(nodes) => nodes
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
            Tree::Feature(_, next) => next.max_depth() + 1,
            Tree::Branch(nodes) => nodes
                .iter()
                .fold(1, |acc, node| acc.max(node.max_depth() + 1)),
        }
    }

    pub fn into_room(self) -> F::Room {
        match self {
            Tree::Feature(feature, child) => {
                let mut room = child.into_room();
                if feature.apply(&mut room) {
                    room
                } else {
                    let mut room = F::Room::default().add_exits(vec![room]);
                    feature.apply(&mut room);
                    room
                }
            }
            Tree::Branch(nodes) => {
                let nodes: Vec<_> = nodes.into_iter().map(|node| node.into_room()).collect();
                F::Room::default().add_exits(nodes)
            }
        }
    }
}

impl<F> Tree<F>
where
    F: Feature + Debug,
{
    pub fn show(&self) {
        fn visit<F>(node: &Tree<F>, mark: bool, indent: usize)
        where
            F: Feature + Debug,
        {
            let prefix = if mark { "+ " } else { "  " };
            match node {
                Tree::Feature(e, t) => {
                    println!("{:indent$}{}{:?}", "", prefix, e, indent = indent);
                    visit(t, false, indent);
                }
                Tree::Branch(ts) => {
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

pub trait Compacter<F>
where
    F: Feature,
{
    fn compact<R>(&self, rng: &mut R, tree: Tree<F>) -> Tree<F>
    where
        R: Rng;
}

impl<F> Default for Tree<F>
where
    F: Feature + Debug,
{
    fn default() -> Self {
        Tree::new()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action<F, T>
where
    F: Debug,
    T: Copy,
{
    New(F),
    PrependAny(F),
    PrependEach(F),
    PrependGrouped(F),
    TransformEach(T),
}

impl<F, T> Action<F, T>
where
    F: Debug + Eq + Feature + Hash,
    T: Transform<F>,
{
    pub fn apply<R: Rng>(&self, rng: &mut R, tree: Tree<F>) -> Tree<F> {
        use rand::prelude::SliceRandom;

        match self {
            Action::New(feature) => {
                let feature = Tree::new_feature(*feature);
                match tree {
                    Tree::Branch(mut nodes) => {
                        nodes.push(feature);
                        Tree::Branch(nodes)
                    }
                    _ => Tree::Branch(vec![tree, feature]),
                }
            }
            Action::PrependAny(feature) => match tree {
                Tree::Branch(mut nodes) => {
                    nodes.choose_mut(rng).unwrap().prepend(*feature);
                    Tree::Branch(nodes)
                }
                _ => tree.prepended(*feature),
            },
            Action::PrependEach(feature) => match tree {
                Tree::Branch(mut nodes) => {
                    for ref mut node in &mut nodes {
                        node.prepend(*feature);
                    }
                    Tree::Branch(nodes)
                }
                _ => tree.prepended(*feature),
            },
            Action::PrependGrouped(feature) => tree.prepended(*feature),
            Action::TransformEach(transform) => match tree {
                Tree::Branch(nodes) => Tree::Branch(
                    nodes
                        .into_iter()
                        .map(|node| transform.apply(rng, node))
                        .collect(),
                ),
                _ => transform.apply(rng, tree),
            },
        }
    }
}

pub trait Feature: Copy {
    type Room: Room + Default;

    fn apply(&self, room: &mut Self::Room) -> bool;
}

pub trait Transform<F>: Copy
where
    F: Feature,
{
    fn apply<R: Rng>(&self, rng: &mut R, tree: Tree<F>) -> Tree<F>;
}

pub trait Room: Sized {
    fn add_exits<I>(self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn leaf() -> Tree<()> {
        Tree::Branch(Vec::new())
    }

    pub fn feature(tree: Tree<()>) -> Tree<()> {
        Tree::Feature((), Box::new(tree))
    }

    pub fn branch1(node: Tree<()>) -> Tree<()> {
        Tree::Branch(vec![node])
    }

    pub fn branch2(node1: Tree<()>, node2: Tree<()>) -> Tree<()> {
        Tree::Branch(vec![node1, node2])
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
