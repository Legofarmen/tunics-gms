pub mod core;
pub mod tunics;

use crate::core::build::BuildPlan;
use crate::core::feature::FeaturePlan;
use crate::core::feature::Room as _;
use crate::tunics::room::Room;
use crate::tunics::AugFeature;
use crate::tunics::Feature;
use crate::tunics::Item;
use layout::Layout;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// PRNG seed
    seed: Option<u64>,

    #[structopt(long, default_value)]
    fairies: usize,

    #[structopt(long, default_value)]
    cul_de_sacs: usize,

    #[structopt(long, default_value)]
    small_keys: usize,

    #[structopt(long)]
    items: Option<Vec<Item>>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    BuildPlan,
    FeaturePlan1,
    FeaturePlan2,
    RoomPlan,
    FloorPlan,
}

mod layout {
    use crate::tunics::room::Room;
    use std::collections::HashMap;

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
        pub fn from_room(room: Room) -> Layout {
            let mut layout = Layout::default();
            walk(&mut layout, room, 0, 0);
            layout
        }
    }
    pub fn walk(layout: &mut Layout, mut room: Room, mut depth: i8, lane0: i8) -> i8 {
        fn room_label(room: &Room) -> String {
            room.contents
                .as_ref()
                .map(|contents| format!("{:?}", contents))
                .unwrap_or_else(|| "".to_string())
        }
        fn entrance_label(room: &Room) -> String {
            room.entrance
                .as_ref()
                .map(|l| format!("{:?}", l))
                .unwrap_or_else(String::new)
        }
        let desc = room_label(&room);
        layout.add_room((depth, lane0), desc);
        room.exits.sort_by_key(Room::weight);
        let last_exit = room.exits.pop();
        for child in room.exits {
            let old_child = depth;
            let entrance = entrance_label(&child);
            depth = walk(layout, child, depth + 1, lane0 + 1);
            for child_depth in (old_child + 1)..=depth {
                layout.add_room((child_depth, lane0), "");
                layout.add_door((child_depth, lane0, Dir4::South), "");
            }
            layout.add_door((old_child + 1, lane0 + 1, Dir4::West), entrance);
        }
        if let Some(last_exit) = last_exit {
            let old_child = depth;
            let entrance = entrance_label(&last_exit);
            depth = walk(layout, last_exit, depth + 1, lane0);
            layout.add_door((old_child + 1, lane0, Dir4::South), entrance);
        }
        depth
    }
}

fn build_plan(seed: u64, opt: Opt) -> (impl Rng, BuildPlan<AugFeature>) {
    use crate::tunics::gen_treasure_set;
    use crate::tunics::Config;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(seed);
    let items = opt
        .items
        .map(|items| items.into_iter().collect())
        .unwrap_or_else(|| gen_treasure_set(&mut rng, 1));
    (
        rng,
        BuildPlan::from(Config {
            num_fairies: opt.fairies,
            num_cul_de_sacs: opt.cul_de_sacs,
            num_small_keys: opt.small_keys,
            items,
        }),
    )
}

fn feature_plan1(seed: u64, opt: Opt) -> (impl Rng, FeaturePlan<AugFeature>) {
    use crate::tunics::get_join_selector;
    use crate::tunics::get_prepend_selector;
    use crate::tunics::get_traversal_selector;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let (mut rng, build_plan) = build_plan(seed, opt);

    let rng1 = StdRng::seed_from_u64(rng.gen());
    let rng2 = StdRng::seed_from_u64(rng.gen());
    let rng3 = StdRng::seed_from_u64(rng.gen());

    let traversal_selector = get_traversal_selector(rng1, &build_plan);
    let prepend_selector = get_prepend_selector(rng2);
    let join_selector = get_join_selector(rng3);
    let build_sequence = build_plan
        .build_sequence(traversal_selector)
        //.inspect(|step| eprintln!("{:?}", step))
        //;
        ;
    (
        rng,
        FeaturePlan::from_steps(join_selector, prepend_selector, build_sequence),
    )
}

fn feature_plan2(seed: u64, opt: Opt) -> (impl Rng, FeaturePlan<Feature>) {
    use crate::tunics::lower;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let (mut rng, feature_plan) = feature_plan1(seed, opt);
    let mut rng4 = StdRng::seed_from_u64(rng.gen());

    (rng, lower(&mut rng4, feature_plan))
}

fn room_plan(seed: u64, opt: Opt) -> (impl Rng, Room) {
    let (rng, feature_plan) = feature_plan2(seed, opt);
    (rng, Room::from_feature_plan(feature_plan))
}

fn floor_plan(seed: u64, opt: Opt) -> (impl Rng, Layout) {
    let (rng, room_plan) = room_plan(seed, opt);
    (rng, Layout::from_room(room_plan))
}

fn main() {
    use rand::rngs::ThreadRng;

    let opt = Opt::from_args();
    let seed = opt.seed.unwrap_or_else(|| ThreadRng::default().gen());
    eprintln!("{}", seed);

    match opt.cmd {
        Command::BuildPlan => {
            build_plan(seed, opt).1.show();
        }
        Command::FeaturePlan1 => {
            feature_plan1(seed, opt).1.show();
        }
        Command::FeaturePlan2 => {
            feature_plan2(seed, opt).1.show();
        }
        Command::RoomPlan => {
            room_plan(seed, opt).1.show();
        }
        Command::FloorPlan => {
            floor_plan(seed, opt).1.show();
        }
    };
}
