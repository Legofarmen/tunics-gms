use crate::core::feature::FeaturePlan;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Tree<D, C> {
    pub entrance: Option<D>,
    pub contents: Option<C>,
    pub exits: Forest<D, C>,
}

#[derive(Debug)]
pub struct Forest<D, C>(VecDeque<Tree<D, C>>);

impl<D, C> fmt::Display for Tree<D, C>
where
    D: fmt::Display,
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(entrance) = &self.entrance {
            write!(f, "{}", entrance)?;
        }
        write!(f, ":")?;
        if let Some(contents) = &self.contents {
            write!(f, "{}", contents)?;
        }
        write!(f, ":{}", self.exits)
    }
}

impl<D, C> fmt::Display for Forest<D, C>
where
    D: fmt::Display,
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut trees = self.0.iter();
        if let Some(tree) = trees.next() {
            write!(f, "{}", tree)?;
        }
        for tree in trees {
            write!(f, ",{}", tree)?;
        }
        write!(f, "]")
    }
}

impl<D, C, I: IntoIterator<Item = Tree<D, C>>> From<I> for Forest<D, C> {
    fn from(trees: I) -> Self {
        Forest(trees.into_iter().collect())
    }
}

impl<D, C> Forest<D, C> {
    pub fn new() -> Self {
        Forest(VecDeque::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_iter(self) -> impl DoubleEndedIterator<Item = Tree<D, C>> {
        self.0.into_iter()
    }
}

impl<D, C> Forest<D, C>
where
    Tree<D, C>: RoomExt,
{
    pub fn sort_by_weight(&mut self) {
        self.0.make_contiguous().sort_by_key(Tree::weight);
    }

    pub fn weights(&self) -> (usize, usize, usize) {
        let (tot, bnd, app) = self.0.iter().map(|tree| tree.weights()).fold(
            (0, 0, Some(0)),
            |(acc_tot, acc_bnd, acc_app), (tot, bnd, app)| {
                let new_app = match (acc_app, app) {
                    (Some(0), app) => Some(app),
                    (app, 0) => app,
                    _ => None,
                };
                (acc_tot + tot, acc_bnd + bnd, new_app)
            },
        );
        (tot, bnd, app.unwrap_or(0))
    }

    pub fn weight(&self) -> usize {
        let (tot, _, _) = self.weights();
        tot
    }

    pub fn linear_weight(&self) -> usize {
        let (_, bnd, app) = self.weights();
        bnd + app
    }
}

impl<D, C> Forest<D, C>
where
    Tree<D, C>: RoomExt,
    D: fmt::Display,
    C: fmt::Display,
{
    pub fn split2(mut self) -> (Self, Self) {
        self.0
            .make_contiguous()
            .sort_by_key(|tree| tree.exits.linear_weight());
        let mut a = Forest::new();
        let mut b = Forest::new();
        let mut a_score = 0;
        let mut b_score = 0;
        for tree in self.0.into_iter().rev() {
            a_score += tree.linear_weight();
            a.0.push_back(tree);
            if a_score > b_score {
                std::mem::swap(&mut a, &mut b);
                std::mem::swap(&mut a_score, &mut b_score);
            }
        }
        (b, a)
    }

    pub fn split3(mut self) -> (Self, Self, Self) {
        self.0
            .make_contiguous()
            .sort_by_key(|tree| tree.exits.linear_weight());
        let mut a = Forest::new();
        let mut b = Forest::new();
        let mut c = Forest::new();
        let mut a_score = 0;
        let mut b_score = 0;
        let mut c_score = 0;
        eprintln!("split3 {}", &self);
        for tree in self.0.into_iter().rev() {
            //eprintln!("- {}", &tree);
            a_score += tree.linear_weight();
            a.0.push_back(tree);
            if a_score > b_score {
                std::mem::swap(&mut a, &mut b);
                std::mem::swap(&mut a_score, &mut b_score);
            }
            if b_score > c_score {
                std::mem::swap(&mut b, &mut c);
                std::mem::swap(&mut b_score, &mut c_score);
            }
            eprintln!("= {} {} {}", a_score, b_score, c_score);
        }
        (a, c, b)
    }

    /// Pop an item that could be taken out and put in front of the entire forest without
    /// changingen the semantics.
    pub fn pop_tree(mut self) -> Result<(Option<D>, Option<C>, Self), Self> {
        if self.0.len() == 1 {
            let Tree {
                entrance,
                contents,
                exits,
            } = self.0.pop_front().unwrap();
            return Ok((entrance, contents, exits));
        }
        self.0.make_contiguous().sort_by_key(Tree::weight);
        if self
            .0
            .get(0)
            .map(|tree| !tree.is_boundary() && tree.weight() == 1)
            .unwrap_or(false)
        {
            let Tree {
                entrance, contents, ..
            } = self.0.pop_front().unwrap();
            Ok((entrance, contents, self))
        } else {
            Err(self)
        }
    }
}

pub trait RoomExt: Default {
    type Feature: fmt::Debug;

    fn add_feature(self, feature: Self::Feature) -> Result<Self, (Self::Feature, Self)>;

    fn is_boundary(&self) -> bool;
}

impl<D, C> Default for Tree<D, C> {
    fn default() -> Self {
        Tree {
            entrance: None,
            contents: None,
            exits: Forest::new(),
        }
    }
}

impl<D, C> Tree<D, C> {
    pub fn add_exits<I>(mut self, exits: I) -> Self
    where
        I: IntoIterator<Item = Tree<D, C>>,
    {
        self.exits.0.extend(exits);
        self
    }
}

impl<D, C> Tree<D, C>
where
    Self: RoomExt,
{
    pub fn add_feature_retry(self, feature: <Self as RoomExt>::Feature) -> Self {
        let (Ok(room) | Err(room)) = self.add_feature(feature).map_err(|(feature, room)| {
            Self::default()
                .add_exits(vec![room])
                .add_feature(feature)
                .map_err(|(feature, _)| panic!("feature: {:?}", feature))
                .ok()
                .unwrap()
        });
        room
    }

    pub fn from_feature_plan(feature_plan: FeaturePlan<<Self as RoomExt>::Feature>) -> Self {
        match feature_plan {
            FeaturePlan::Feature(feature, child) => {
                Self::from_feature_plan(*child).add_feature_retry(feature)
            }
            FeaturePlan::Branch(nodes) => {
                let nodes: Vec<_> = nodes.into_iter().map(Self::from_feature_plan).collect();
                Self::default().add_exits(nodes)
            }
        }
    }

    pub fn weights(&self) -> (usize, usize, usize) {
        let (tot, bnd, app) = self.exits.weights();
        if self.is_boundary() {
            (tot + 1, 0, bnd + app + 1)
        } else {
            (tot + 1, bnd + 1, app)
        }
    }

    pub fn weight(&self) -> usize {
        let (tot, _, _) = self.weights();
        tot
    }

    pub fn linear_weight(&self) -> usize {
        let (_, bnd, app) = self.weights();
        bnd + app
    }
}

impl<D: fmt::Display, C: fmt::Display> Tree<D, C> {
    pub fn show<M: std::fmt::Display>(&self, metadata: M, seed: u64) {
        fn visit<D: fmt::Display, C: fmt::Display>(
            room: &Tree<D, C>,
            parent: usize,
            id: usize,
        ) -> usize {
            let mut next = id + 1;
            for child in &room.exits.0 {
                next = visit(child, id, next);
            }
            let door = match &room.entrance {
                None => "".to_string(),
                Some(door) => format!("{}", door),
            };
            if let Some(contents) = &room.contents {
                println!("  room{} [label=\"{}\"];", id, contents);
            } else {
                println!("  room{} [label=\"\"];", id);
            }
            println!("  room{} -- room{} [label=\"{}\"];", parent, id, door);
            next
        }
        println!("graph {{");
        println!("  labelloc=\"t\";");
        println!(
            "  label=<<b>Room plan</b><br/>{}<br/>seed: {}>;",
            metadata, seed
        );
        visit(self, 0, 0);
        println!("}}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl RoomExt for Tree<(), ()> {
        type Feature = ();
        fn add_feature(self, feature: Self::Feature) -> Result<Self, (Self::Feature, Self)> {
            if self.entrance.is_none() {
                Err((feature, self))
            } else {
                Ok(self)
            }
        }
        fn is_boundary(&self) -> bool {
            self.entrance.is_some()
        }
    }
    fn empty<F: Into<Forest<(), ()>>>(exits: F) -> Tree<(), ()> {
        Tree {
            entrance: None,
            contents: None,
            exits: exits.into(),
        }
    }
    fn boundary<F: Into<Forest<(), ()>>>(exits: F) -> Tree<(), ()> {
        Tree {
            entrance: Some(()),
            contents: None,
            exits: exits.into(),
        }
    }

    #[test]
    fn test_weights() {
        assert_eq!((0, 0, 0), Forest::<(), ()>::new().weights());

        assert_eq!((1, 1, 0), empty([]).weights());
        assert_eq!((1, 0, 1), boundary([]).weights());

        assert_eq!((2, 2, 0), empty([empty([])]).weights());
        assert_eq!((2, 1, 1), empty([boundary([])]).weights());
        assert_eq!((2, 0, 2), boundary([empty([])]).weights());
        assert_eq!((2, 0, 2), boundary([boundary([])]).weights());

        assert_eq!((3, 3, 0), empty([empty([]), empty([])]).weights());
        assert_eq!((3, 2, 1), empty([empty([]), boundary([])]).weights());
        assert_eq!((3, 1, 0), empty([boundary([]), boundary([])]).weights());
        assert_eq!((3, 0, 1), boundary([boundary([]), boundary([])]).weights());
        assert_eq!((3, 0, 3), boundary([empty([]), empty([])]).weights());
        assert_eq!((3, 0, 3), boundary([empty([]), boundary([])]).weights());

        assert_eq!(
            (4, 4, 0),
            empty([empty([]), empty([]), empty([])]).weights()
        );
        assert_eq!(
            (4, 3, 1),
            empty([empty([]), empty([]), boundary([])]).weights()
        );
        assert_eq!(
            (4, 2, 0),
            empty([empty([]), boundary([]), boundary([])]).weights()
        );
        assert_eq!(
            (4, 0, 4),
            boundary([empty([]), empty([]), empty([])]).weights()
        );
        assert_eq!(
            (4, 0, 4),
            boundary([empty([]), empty([]), boundary([])]).weights()
        );
        assert_eq!(
            (4, 0, 2),
            boundary([empty([]), boundary([]), boundary([])]).weights()
        );
        assert_eq!(
            (4, 0, 1),
            boundary([boundary([]), boundary([]), boundary([])]).weights()
        );
    }
}
