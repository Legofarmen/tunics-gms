use crate::core::feature::FeaturePlan;
use std::fmt::Debug;

pub struct Room<D, C> {
    pub entrance: Option<D>,
    pub contents: Option<C>,
    pub exits: Vec<Self>,
}

pub trait RoomExt: Default {
    type Feature: Debug;

    fn add_exits<I>(self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>;

    fn add_feature(self, feature: Self::Feature) -> Result<Self, (Self::Feature, Self)>;

    fn add_feature_retry(self, feature: Self::Feature) -> Self {
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

    fn from_feature_plan(feature_plan: FeaturePlan<Self::Feature>) -> Self {
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

impl<D, C> Default for Room<D, C> {
    fn default() -> Self {
        Room {
            entrance: None,
            contents: None,
            exits: vec![],
        }
    }
}

impl<D, C> Room<D, C> {
    pub fn weight(&self) -> usize {
        self.exits
            .iter()
            .map(Room::weight)
            .fold(1, |acc, weight| acc + weight)
    }
}

impl<D: Debug, C: Debug> Room<D, C> {
    pub fn show(&self) {
        fn visit<D: Debug, C: Debug>(room: &Room<D, C>, parent: usize, id: usize) -> usize {
            let mut next = id + 1;
            for child in &room.exits {
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
