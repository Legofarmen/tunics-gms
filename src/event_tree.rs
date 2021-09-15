use rand::Rng;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Visitor<E>
where
    E: Event,
{
    fn visit_event(&mut self, event: &E, next: Tree<E>) -> E::Room;

    fn visit_branch(&mut self, nodes: Vec<Tree<E>>) -> E::Room;
}

#[derive(Clone)]
pub enum Tree<E>
where
    E: Event,
{
    Event(E, Box<Tree<E>>),
    Branch(Vec<Tree<E>>),
}

impl<E> Tree<E>
where
    E: Event,
{
    pub fn new() -> Self {
        Tree::Branch(Vec::new())
    }

    pub fn new_event(event: E) -> Self {
        Tree::new().prepended(event)
    }

    pub fn join(&mut self, other: Self) {
        match (self, other) {
            (Tree::Branch(ref mut u), Tree::Branch(mut v)) => {
                u.append(&mut v);
            }
            (Tree::Branch(ref mut u), event) => {
                u.push(event);
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

    pub fn prepend(&mut self, event: E) {
        *self = Tree::Event(event, Box::new(self.clone()));
    }

    pub fn prepended(self, event: E) -> Self {
        Tree::Event(event, Box::new(self))
    }

    pub fn skip_event(&mut self) {
        if let Tree::Event(_, next) = self {
            *self = (**next).clone();
        }
    }

    pub fn find_event_depth<'a, P>(&'a self, predicate: &P) -> Option<usize>
    where
        P: Fn(&'a E) -> bool,
        E: 'a,
    {
        match self {
            Tree::Event(event, next) => {
                if predicate(event) {
                    Some(0)
                } else {
                    next.find_event_depth(predicate).map(|depth| depth + 1)
                }
            }
            Tree::Branch(nodes) => nodes
                .iter()
                .map(|node| node.find_event_depth(predicate).map(|depth| depth + 1))
                .fold(None, |acc, depth| match (acc, depth) {
                    (Some(acc), Some(depth)) => Some(acc.min(depth)),
                    (acc, depth) => acc.or(depth),
                }),
        }
    }

    pub fn max_depth(&self) -> usize {
        match self {
            Tree::Event(_, next) => next.max_depth() + 1,
            Tree::Branch(nodes) => nodes
                .iter()
                .fold(1, |acc, node| acc.max(node.max_depth() + 1)),
        }
    }

    pub fn accept<V>(self, visitor: &mut V) -> E::Room
    where
        V: Visitor<E>,
    {
        match self {
            Tree::Event(event, next) => visitor.visit_event(&event, (*next).clone()),
            Tree::Branch(nodes) => visitor.visit_branch(nodes),
        }
    }

    pub fn into_room(self) -> E::Room {
        struct V;

        impl<E: Event> Visitor<E> for V {
            fn visit_event(&mut self, event: &E, next: Tree<E>) -> E::Room {
                let mut room = next.accept(self);
                if event.apply(&mut room) {
                    room
                } else {
                    let mut room = E::Room::default().add_exits(vec![room]);
                    event.apply(&mut room);
                    room
                }
            }

            fn visit_branch(&mut self, nodes: Vec<Tree<E>>) -> E::Room {
                let nodes: Vec<_> = nodes.into_iter().map(|node| node.accept(self)).collect();
                E::Room::default().add_exits(nodes)
            }
        }
        self.accept(&mut V)
    }
}

impl<E> Tree<E>
where
    E: Event + Debug,
{
    pub fn show(&self) {
        fn visit<E>(node: &Tree<E>, mark: bool, indent: usize)
        where
            E: Event + Debug,
        {
            let prefix = if mark { "+ " } else { "  " };
            match node {
                Tree::Event(e, t) => {
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

pub trait Compacter<E>
where
    E: Event,
{
    fn compact<R>(&self, rng: &mut R, tree: Tree<E>) -> Tree<E>
    where
        R: Rng;
}

impl<E> Default for Tree<E>
where
    E: Event + Debug,
{
    fn default() -> Self {
        Tree::new()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action<E, T>
where
    E: Debug,
    T: Copy,
{
    New(E),
    PrependAny(E),
    PrependEach(E),
    PrependGrouped(E),
    TransformEach(T),
}

impl<E, T> Action<E, T>
where
    E: Debug + Eq + Event + Hash,
    T: Transform<E>,
{
    pub fn apply<R: Rng>(&self, rng: &mut R, mut tree: Tree<E>) -> Tree<E> {
        use rand::prelude::SliceRandom;

        match self {
            Action::New(event) => {
                let event = Tree::new_event(*event);
                match tree {
                    Tree::Branch(mut nodes) => {
                        nodes.push(event);
                        Tree::Branch(nodes)
                    }
                    _ => Tree::Branch(vec![tree, event]),
                }
            }
            Action::PrependAny(event) => match tree {
                Tree::Branch(mut nodes) => {
                    nodes.choose_mut(rng).unwrap().prepend(*event);
                    Tree::Branch(nodes)
                }
                _ => tree.prepended(*event),
            },
            Action::PrependEach(event) => match tree {
                Tree::Branch(mut nodes) => {
                    for ref mut node in &mut nodes {
                        node.prepend(*event);
                    }
                    Tree::Branch(nodes)
                }
                _ => tree.prepended(*event),
            },
            Action::PrependGrouped(event) => tree.prepended(*event),
            Action::TransformEach(transform) => match tree {
                Tree::Branch(mut nodes) => {
                    for ref mut node in &mut nodes {
                        transform.apply(rng, node);
                    }
                    Tree::Branch(nodes)
                }
                _ => {
                    transform.apply(rng, &mut tree);
                    tree
                }
            },
        }
    }
}

pub trait Event: Copy {
    type Room: Room + Default;

    fn apply(&self, room: &mut Self::Room) -> bool;
}

pub trait Transform<E>: Copy
where
    E: Event,
{
    fn apply<R: Rng>(&self, rng: &mut R, tree: &mut Tree<E>);
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

    pub fn event(tree: Tree<()>) -> Tree<()> {
        Tree::Event((), Box::new(tree))
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
        assert_eq!(2, event(leaf()).max_depth());
        assert_eq!(3, event(event(leaf())).max_depth());

        assert_eq!(
            7,
            branch2(
                event(leaf()),
                event(branch2(
                    event(event(event(leaf()))),
                    event(event(event(leaf()))),
                )),
            )
            .max_depth()
        );
    }
}
