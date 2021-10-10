use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir2 {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DoorCoord2 {
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

impl DoorCoord4 {
    pub fn neighbour(self) -> Coord {
        match self.dir {
            Dir4::North => self.coord.north(),
            Dir4::East => self.coord.east(),
            Dir4::South => self.coord.south(),
            Dir4::West => self.coord.west(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

    pub fn show<M: std::fmt::Display>(&self, metadata: M, seed: u64) {
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
        println!("  labelloc=\"t\";");
        println!(
            "  label=<<b>Floor plan</b><br/>{}<br/>seed: {}>;",
            metadata, seed,
        );
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
}
