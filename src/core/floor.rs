use crate::core::room::Tree as RoomTree;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Dir2 {
    North,
    East,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Dir4 {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coord {
    x: i8,
    y: i8,
}

impl From<(i8, i8)> for Coord {
    fn from((y, x): (i8, i8)) -> Self {
        Coord { y, x }
    }
}

impl Coord {
    pub fn north(self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn south(self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn east(self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn west(self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct DoorCoord2 {
    coord: Coord,
    dir: Dir2,
}

impl DoorCoord2 {
    pub fn neighbour(self) -> Coord {
        match self.dir {
            Dir2::North => self.coord.north(),
            Dir2::East => self.coord.east(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DoorCoord4 {
    coord: Coord,
    dir: Dir4,
}

impl From<DoorCoord4> for DoorCoord2 {
    fn from(DoorCoord4 { coord, dir }: DoorCoord4) -> Self {
        match dir {
            Dir4::North => DoorCoord2 {
                coord,
                dir: Dir2::North,
            },
            Dir4::East => DoorCoord2 {
                coord,
                dir: Dir2::East,
            },
            Dir4::South => DoorCoord2 {
                coord: coord.south(),
                dir: Dir2::North,
            },
            Dir4::West => DoorCoord2 {
                coord: coord.west(),
                dir: Dir2::East,
            },
        }
    }
}

impl From<(i8, i8, Dir4)> for DoorCoord4 {
    fn from((y, x, dir): (i8, i8, Dir4)) -> Self {
        let coord = (y, x).into();
        DoorCoord4 { coord, dir }
    }
}

#[derive(Default)]
pub struct FloorPlan {
    rooms: HashMap<Coord, Option<String>>,
    doors: HashMap<DoorCoord2, String>,
}

impl FloorPlan {
    pub fn add_room<C, S>(&mut self, coord: C, desc: Option<S>)
    where
        C: Into<Coord>,
        S: Into<String>,
    {
        use std::collections::hash_map::Entry;
        let coord = coord.into();
        match self.rooms.entry(coord) {
            Entry::Occupied(_) => {
                panic!("room already exist: ({};{})", coord.x, coord.y);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(desc.map(S::into));
            }
        }
    }

    pub fn add_door<C, S>(&mut self, door_coord4: C, desc: S)
    where
        C: Into<DoorCoord4>,
        S: Into<String>,
    {
        use std::collections::hash_map::Entry;

        let door_coord4 = door_coord4.into();
        let door_coord2 = DoorCoord2::from(door_coord4);
        if !self.rooms.contains_key(&door_coord2.coord) {
            panic!(
                "room doest not exist: ({};{})",
                door_coord2.coord.x, door_coord2.coord.y
            );
        }
        if !self.rooms.contains_key(&door_coord2.neighbour()) {
            panic!(
                "room doest not exist: ({};{})",
                door_coord2.neighbour().x,
                door_coord2.neighbour().y
            );
        }
        match self.doors.entry(door_coord2) {
            Entry::Occupied(_) => {
                panic!(
                    "door already exist: ({};{};{:?})",
                    door_coord4.coord.x, door_coord4.coord.y, door_coord4.dir
                );
            }
            Entry::Vacant(vacant) => {
                vacant.insert(desc.into());
            }
        }
    }

    pub fn show(&self) {
        fn room_name(coord: Coord) -> String {
            if coord.y >= 0 {
                if coord.x >= 0 {
                    format!("n{}e{}", coord.y, coord.x)
                } else {
                    format!("n{}w{}", coord.y, -coord.x)
                }
            } else {
                if coord.x >= 0 {
                    format!("s{}e{}", -coord.y, coord.x)
                } else {
                    format!("s{}w{}", -coord.y, -coord.x)
                }
            }
        }
        println!("graph {{");
        println!("  layout=neato;");
        println!("  node [shape=record, width=\"1\", height=\"1\"];");
        for (coord, desc) in &self.rooms {
            if let Some(desc) = desc {
                println!(
                    "  {} [pos=\"{},{}!\", label=\"{}\"];",
                    room_name(*coord),
                    2 * coord.x,
                    2 * coord.y,
                    desc,
                );
            } else {
                println!(
                    "  {} [pos=\"{},{}!\", label=\"\" color=\"white\"];",
                    room_name(*coord),
                    2 * coord.x,
                    2 * coord.y,
                );
            }
        }
        for (coord, desc) in &self.doors {
            println!(
                "  {} -- {} [label=\"{}\"];",
                room_name(coord.coord),
                room_name(coord.neighbour()),
                desc
            );
        }
        println!("}}");
    }

    pub fn from_room<D: Debug, C: Debug>(room: RoomTree<D, C>) -> FloorPlan {
        let mut floor_plan = FloorPlan::default();
        let entrance = entrance_label(&room);
        floor_plan.walk(room, 0, 0);
        floor_plan.add_room::<_, String>((-1, 0), None);
        floor_plan.add_door((0, 0, Dir4::South), entrance);
        floor_plan
    }

    fn walk<D: Debug, C: Debug>(
        &mut self,
        mut room: RoomTree<D, C>,
        mut depth: i8,
        lane0: i8,
    ) -> i8 {
        fn room_label<D, C: Debug>(room: &RoomTree<D, C>) -> String {
            room.contents
                .as_ref()
                .map(|contents| format!("{:?}", contents))
                .unwrap_or_else(|| "".to_string())
        }
        let desc = room_label(&room);
        self.add_room((depth, lane0), Some(desc));
        room.exits.sort_by_weight();
        let mut exits = room.exits.into_iter();
        let last_exit = exits.next_back();
        for child in exits {
            let old_child = depth;
            let entrance = entrance_label(&child);
            depth = self.walk(child, depth + 1, lane0 + 1);
            for child_depth in (old_child + 1)..=depth {
                self.add_room((child_depth, lane0), Some(""));
                self.add_door((child_depth, lane0, Dir4::South), "");
            }
            self.add_door((old_child + 1, lane0 + 1, Dir4::West), entrance);
        }
        if let Some(last_exit) = last_exit {
            let old_child = depth;
            let entrance = entrance_label(&last_exit);
            depth = self.walk(last_exit, depth + 1, lane0);
            self.add_door((old_child + 1, lane0, Dir4::South), entrance);
        }
        depth
    }
}

fn entrance_label<D: Debug, C>(room: &RoomTree<D, C>) -> String {
    room.entrance
        .as_ref()
        .map(|l| format!("{:?}", l))
        .unwrap_or_else(String::new)
}
