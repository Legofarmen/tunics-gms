use rand::Rng;

pub trait Action {
    type Item;
    fn apply(&self, heads: &mut Vec<Tree<Self::Item>>);
}

pub enum Tree<T> {
    Event(T, Box<Tree<T>>),
    Branch(Vec<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn from_actions<R, A, W>(rng: &mut R, max_heads: usize, actions: &[A], w: W) -> Tree<T>
    where
        R: Rng,
        W: Fn(&Tree<T>) -> Box<dyn Fn(&Tree<T>) -> u32>,
        A: Action<Item = T>,
    {
        use rand::distributions::weighted::WeightedIndex;
        use rand::distributions::Distribution;

        let mut heads = Vec::new();
        for action in actions {
            while heads.len() > max_heads {
                let head1 = heads.remove(rng.gen_range(0..heads.len()));
                let calc_join_weight = w(&head1);
                let dist = WeightedIndex::new(heads.iter().map(calc_join_weight)).unwrap();
                let head2 = heads.remove(dist.sample(rng));
                heads.push(head1.join(head2));
            }
            action.apply(&mut heads);
        }
        Tree::Branch(heads)
    }

    pub fn join(self, other: Self) -> Self {
        match (self, other) {
            (Tree::Branch(mut u), Tree::Branch(mut v)) => {
                u.extend(v.drain(..));
                Tree::Branch(u)
            }
            (Tree::Branch(mut u), e) => {
                u.push(e);
                Tree::Branch(u)
            }
            (e, Tree::Branch(mut u)) => {
                u.push(e);
                Tree::Branch(u)
            }
            (e, f) => Tree::Branch(vec![e, f]),
        }
    }
}

impl<T: std::fmt::Debug> Tree<T> {
    pub fn show(&self) {
        fn visit<T: std::fmt::Debug>(node: &Tree<T>, indent: usize) {
            match node {
                Tree::Event(e, t) => {
                    println!("{:indent$}{:?}", "", e, indent = indent);
                    visit(t, indent + 2);
                }
                Tree::Branch(ts) => {
                    println!("{:indent$}+", "", indent = indent);
                    for t in ts {
                        visit(t, indent + 2);
                    }
                }
            }
        }
        visit(self, 0);
    }
}
