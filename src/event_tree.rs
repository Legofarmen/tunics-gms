use rand::Rng;

pub trait Action {
    type Item;
    fn apply<R: Rng>(&self, rng: &mut R, heads: &mut Vec<Tree<Self::Item>>);
}

#[derive(Clone, Debug)]
pub enum Tree<E> {
    Event(E, Box<Tree<E>>),
    Branch(Vec<Tree<E>>),
}

impl<E> Tree<E>
where
    E: Copy,
{
    pub fn new() -> Self {
        Tree::Branch(Vec::new())
    }

    pub fn from_actions<R, A, W>(rng: &mut R, max_heads: usize, actions: &[A], w: W) -> Tree<E>
    where
        R: Rng,
        W: Fn(&Tree<E>, usize) -> Box<dyn Fn(&Tree<E>) -> usize>,
        A: Action<Item = E>,
    {
        use rand::distributions::weighted::WeightedIndex;
        use rand::distributions::Distribution;

        let mut heads = Vec::new();
        for action in actions {
            while heads.len() > max_heads {
                let head = heads.remove(rng.gen_range(0..heads.len()));
                let max_depth = heads
                    .iter()
                    .fold(0, |acc: usize, node: &Tree<E>| acc.max(node.max_depth()));
                let calc_join_weight = w(&head, max_depth);
                let dist = WeightedIndex::new(heads.iter().map(calc_join_weight)).unwrap();
                heads.get_mut(dist.sample(rng)).unwrap().join(head);
            }
            action.apply(rng, &mut heads);
        }
        if heads.len() == 1 {
            heads.pop().unwrap()
        } else {
            Tree::Branch(heads)
        }
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
}

impl<E: std::fmt::Debug> Tree<E> {
    pub fn show(&self) {
        fn visit<E: std::fmt::Debug>(node: &Tree<E>, mark: bool, indent: usize) {
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

impl<E> Default for Tree<E>
where
    E: Copy,
{
    fn default() -> Self {
        Tree::new()
    }
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
