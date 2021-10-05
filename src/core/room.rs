use std::fmt::Debug;

pub struct Room<D, C> {
    pub entrance: Option<D>,
    pub contents: Option<C>,
    pub exits: Vec<Self>,
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
