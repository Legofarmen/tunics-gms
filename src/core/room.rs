use crate::core::feature::FeaturePlan;
use std::collections::VecDeque;
use std::fmt::Debug;

pub struct Tree<D, C> {
    pub entrance: Option<D>,
    pub contents: Option<C>,
    pub exits: Forest<D, C>,
}

pub struct Forest<D, C>(VecDeque<Tree<D, C>>);

pub struct Segment<D, C>(VecDeque<Forest<D, C>>);

impl<D, C> Segment<D, C> {
    pub fn split_off(&mut self, at: usize) -> Self {
        Segment(self.0.split_off(at))
    }
    pub fn pop_left(&mut self) -> Option<Forest<D, C>> {
        self.0.pop_front()
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
    pub fn sort_by_weight(&mut self) {
        self.0.make_contiguous().sort_by_key(Tree::weight);
    }

    pub fn into_iter(self) -> impl DoubleEndedIterator<Item = Tree<D, C>> {
        self.0.into_iter()
    }

    pub fn weight(&self) -> usize {
        self.0
            .iter()
            .map(|tree| tree.exits.weight())
            .fold(1, |acc, weight| acc + weight)
    }
}

impl<D, C> Forest<D, C>
where
    Tree<D, C>: RoomExt,
{
    pub fn linear_weight(&self) -> usize {
        if self.0.iter().filter(|room| room.is_boundary()).count() > 1 {
            1
        } else {
            self.0
                .iter()
                .fold(1, |weight, room| weight + room.exits.linear_weight())
        }
    }
}

impl<D, C> Forest<D, C>
where
    Tree<D, C>: RoomExt,
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
            a_score += tree.exits.linear_weight();
            a.0.push_back(tree);
            if a_score > b_score {
                std::mem::swap(&mut a, &mut b);
                std::mem::swap(&mut a_score, &mut b_score);
            }
        }
        (a, b)
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
        for tree in self.0.into_iter().rev() {
            a_score += tree.exits.linear_weight();
            a.0.push_back(tree);
            if a_score > b_score {
                std::mem::swap(&mut a, &mut b);
                std::mem::swap(&mut a_score, &mut b_score);
            }
            if b_score > c_score {
                std::mem::swap(&mut b, &mut c);
                std::mem::swap(&mut b_score, &mut c_score);
            }
        }
        (a, b, c)
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
    type Feature: Debug;

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

    pub fn weight(&self) -> usize {
        self.exits
            .0
            .iter()
            .map(|tree| tree.exits.weight())
            .fold(1, |acc, weight| acc + weight)
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
}

impl<D: Debug, C: Debug> Tree<D, C> {
    pub fn show(&self) {
        fn visit<D: Debug, C: Debug>(room: &Tree<D, C>, parent: usize, id: usize) -> usize {
            let mut next = id + 1;
            for child in &room.exits.0 {
                next = visit(child, id, next);
            }
            let door = match &room.entrance {
                None => "".to_string(),
                Some(door) => format!("{:?}", door),
            };
            if let Some(contents) = &room.contents {
                println!("  room{} [label=\"{:?}\"];", id, contents);
            } else {
                println!("  room{} [label=\"\"];", id);
            }
            println!("  room{} -- room{} [label=\"{}\"];", parent, id, door);
            next
        }
        println!("graph {{");
        visit(self, 0, 0);
        println!("}}");
    }
}
