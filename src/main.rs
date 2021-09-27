pub mod core;
pub mod tunics;

trait Check<T> {
    fn check<F>(self, f: F) -> T
    where
        F: Fn(&T);
}

impl<T> Check<T> for T {
    fn check<F>(self, f: F) -> T
    where
        F: Fn(&T),
    {
        f(&self);
        self
    }
}

mod layout {
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::ops::RangeInclusive;

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum Dir2 {
        North,
        East,
    }

    #[derive(Clone, Copy, Debug)]
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
    pub struct Layout {
        rooms: HashMap<Coord, String>,
        doors: HashMap<DoorCoord2, String>,
    }

    impl Layout {
        pub fn add_room<C, S>(&mut self, coord: C, desc: S)
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
                    vacant.insert(desc.into());
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
        pub fn bounds(&self) -> (RangeInclusive<i8>, RangeInclusive<i8>) {
            use itertools::MinMaxResult;

            fn as_range(minmax: MinMaxResult<i8>) -> RangeInclusive<i8> {
                match minmax {
                    MinMaxResult::NoElements => 0..=0,
                    MinMaxResult::OneElement(i) => i..=i,
                    MinMaxResult::MinMax(min, max) => min..=max,
                }
            }
            let minmax_x = self.rooms.keys().map(|c| c.x).minmax();
            let minmax_y = self.rooms.keys().map(|c| c.y).minmax();
            (as_range(minmax_x), as_range(minmax_y))
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
            println!("  rankdir=BT;");
            println!("  node [shape=record, width=\"1\", height=\"1\"];");
            for (coord, desc) in &self.rooms {
                println!(
                    "  {} [pos=\"{},{}!\", label=\"{}\"];",
                    room_name(*coord),
                    2 * coord.x,
                    2 * coord.y,
                    desc,
                );
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
}

fn main() {
    use crate::core::build::BuildPlan;
    use crate::core::feature::FeaturePlan;
    use crate::core::feature::Room as _;
    use crate::tunics::get_join_selector;
    use crate::tunics::get_prepend_selector;
    use crate::tunics::get_traversal_selector;
    use crate::tunics::hide_chests;
    use crate::tunics::Config;
    use crate::tunics::Room;
    use crate::tunics::Treasure;
    use rand::rngs::StdRng;
    use rand::rngs::ThreadRng;
    use rand::Rng;
    use rand::SeedableRng;
    use std::env;
    use std::str::FromStr;

    let mut args = env::args().skip(1);
    let seed = args
        .next()
        .map(|s| u64::from_str(&s).expect("seed must be numeric"))
        .unwrap_or_else(|| ThreadRng::default().gen());
    args.next()
        .and_then::<String, _>(|_| panic!("too many argument"));
    //eprintln!("{}", seed);

    let mut rng = StdRng::seed_from_u64(seed);
    let rng1 = StdRng::seed_from_u64(rng.gen());
    let rng2 = StdRng::seed_from_u64(rng.gen());
    let rng3 = StdRng::seed_from_u64(rng.gen());
    let mut rng4 = StdRng::seed_from_u64(rng.gen());

    let build_plan = BuildPlan::from(Config {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    })
    //.check(|build_plan| build_plan.show())
    ;

    let traversal_selector = get_traversal_selector(rng1, &build_plan);
    let prepend_selector = get_prepend_selector(rng2);
    let join_selector = get_join_selector(rng3);
    let build_sequence = build_plan
        .build_sequence(traversal_selector)
        //.inspect(|step| eprintln!("{:?}", step))
        ;
    let feature_plan = FeaturePlan::from_steps(join_selector, prepend_selector, build_sequence)
        //.check(|feature_plan| feature_plan.show())
        ;
    let room = Room::from_feature_plan(hide_chests(&mut rng4, feature_plan)).check(Room::show);

    //pub entrance: Option<Lock>,
    //pub chest: Option<Treasure>,
    //pub far_side_chest: Option<bool>,

    use layout::Dir4;
    fn walk(layout: &mut Layout, room: Room, mut y: i8, x: i8) -> i8 {
        let obstacle = room
            .obstacle
            .map(|o| format!("{:?}", o))
            .unwrap_or_else(String::new);
        let (near_chest, far_chest) = if room.far_side_chest == Some(true) {
            (
                String::new(),
                room.chest
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(String::new),
            )
        } else {
            (
                room.chest
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(String::new),
                String::new(),
            )
        };
        let desc = format!("{{{}|{}|{}}}", far_chest, obstacle, near_chest);
        layout.add_room((y, x), desc);
        for child in room.exits {
            let y0 = y;
            let entrance = child
                .entrance
                .as_ref()
                .map(|l| format!("{:?}", l))
                .unwrap_or_else(String::new);
            y = walk(layout, child, y + 1, x + 1);
            for y1 in (y0 + 1)..=y {
                layout.add_room((y1, x), "");
                layout.add_door((y1, x, Dir4::South), "");
            }
            layout.add_door((y0 + 1, x + 1, Dir4::West), entrance);
        }
        y
    }

    //use layout::Dir4::*;
    use layout::Layout;
    let mut l = Layout::default();
    //l.add_room((0, 0), "{|entrance|}");
    //l.add_room((1, 0), "{|mote|key}");
    //l.add_room((0, 1), "{||}");
    //l.add_door((0, 0, North), "lock");
    //l.add_door((0, 0, East), "");
    walk(&mut l, room, 0, 0);
    l.show();
}
