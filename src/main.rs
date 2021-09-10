pub mod outline;

use rand::Rng;

trait Action {
    type Item;
    fn apply(&self, heads: &mut Vec<Tree<Self::Item>>);
}

enum Tree<T> {
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
                let weighter = w(&head1);
                let dist = WeightedIndex::new(heads.iter().map(weighter)).unwrap();
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

fn main() {
    use outline::OutlineConf;
    use outline::Treasure;

    let mut rng = rand::rngs::mock::StepRng::new(1, 3);
    //let mut rng = rand::rngs::ThreadRng::default();

    let outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
    }
    .into_outline();
    let actions = outline.action_sequence(&mut rng);

    //outline.show();
    for action in &actions {
        println!("{:?}", action);
    }

    let tree = Tree::from_actions(&mut rng, 4, &actions, |_| Box::new(|_| 0));
}
