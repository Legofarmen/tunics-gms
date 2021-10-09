use crate::core::floor::Dir4;
use crate::core::floor::DoorCoord4;
use crate::core::floor::FloorPlan;
use crate::core::room::Forest;
use crate::core::room::RoomExt;
use crate::core::room::Tree;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Level<T> {
    depth: i8,
    cells: Vec<T>,
}

impl<T> Level<T> {
    pub fn set(&mut self, pos: i8, elem: T) {
        self.cells[(self.depth + pos) as usize] = elem;
    }
}

impl<D, C> Level<Forest<D, C>> {
    pub fn new_forests(depth: i8) -> Self {
        let width = 2 * depth as usize + 1;
        let mut forests = Vec::with_capacity(width);
        forests.resize_with(width, || Forest::new());
        Level {
            depth,
            cells: forests,
        }
    }
    pub fn is_empty(&self, pos: i8) -> bool {
        self.cells[(self.depth + pos) as usize].is_empty()
    }
}

impl<D, C> Level<Forest<D, C>>
where
    Tree<D, C>: RoomExt,
{
    pub fn score(&self, pos: i8) -> (isize, isize) {
        let forest = &self.cells[(self.depth + pos) as usize];
        let weight = forest.weight() as isize;
        let x = if weight > 1 {
            -(forest.linear_weight() as isize)
        } else {
            isize::MIN
        };
        (x, -weight)
    }
}

impl<T: Ord> Level<BTreeSet<T>> {
    pub fn new_sets(depth: i8) -> Self {
        let width = 2 * depth as usize + 1;
        let mut sets = Vec::with_capacity(width);
        sets.resize_with(width, || BTreeSet::new());
        Level { depth, cells: sets }
    }
    pub fn insert(&mut self, pos: i8, elem: T) {
        self.cells[(self.depth + pos) as usize].insert(elem);
    }
}

fn allocate<D, C>(
    next_level: &mut Level<Forest<D, C>>,
    forest: Forest<D, C>,
    x: i8,
    y: i8,
    dir: Dir4,
    floor_plan: &mut FloorPlan,
) where
    Tree<D, C>: RoomExt,
    D: ToString,
    C: ToString,
{
    let delta = match dir {
        Dir4::North => 0,
        Dir4::East => 1,
        Dir4::West => -1,
        _ => unreachable!(),
    };
    let forest = match forest.pop_tree() {
        Ok((door, contents, forest)) => {
            let door_label = door
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "".to_string());
            let room_label = contents.as_ref().map(ToString::to_string);
            let door_coord: DoorCoord4 = (y, x, dir).into();
            let room_coord = door_coord.neighbour();
            floor_plan.add_room(room_coord, room_label);
            floor_plan.add_door(door_coord, door_label);
            forest
        }
        Err(forest) => forest,
    };
    next_level.set(x + delta, forest);
}

pub fn from_forest<D, C>(forest: Forest<D, C>) -> FloorPlan
where
    Tree<D, C>: RoomExt,
    D: ToString + std::fmt::Debug,
    C: ToString + std::fmt::Debug,
{
    let mut floor_plan = FloorPlan::default();
    floor_plan.add_room::<_, String>((0, 0), None);

    let mut depth = 0;
    let mut level = Level::<Forest<D, C>>::new_forests(depth);
    level.set(0, forest);
    let mut seen_non_empty = true;
    while seen_non_empty {
        let mut alloc = Level::new_sets(depth);

        eprintln!("\ndepth {}", depth);
        seen_non_empty =
            alloc_half(&level, &mut alloc, -depth..=0) | alloc_half(&level, &mut alloc, 0..=depth);

        let mut next_level = Level::<Forest<D, C>>::new_forests(depth + 1);
        for (i, (forest, alloc)) in level
            .cells
            .into_iter()
            .zip(alloc.cells.into_iter())
            .enumerate()
        {
            let x = i as i8 - depth;
            let y = depth - x.abs();
            if alloc.len() == 0 {
            } else if alloc.len() == 1 {
                let mut alloc = alloc.into_iter();
                allocate(
                    &mut next_level,
                    forest,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
            } else if alloc.len() == 2 {
                let mut alloc = alloc.into_iter();
                let (forest1, forest2) = forest.split2();
                allocate(
                    &mut next_level,
                    forest1,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
                allocate(
                    &mut next_level,
                    forest2,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
            } else if alloc.len() == 3 {
                let mut alloc = alloc.into_iter();
                let (forest1, forest2, forest3) = forest.split3();
                allocate(
                    &mut next_level,
                    forest1,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
                allocate(
                    &mut next_level,
                    forest2,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
                allocate(
                    &mut next_level,
                    forest3,
                    x,
                    y,
                    alloc.next().unwrap(),
                    &mut floor_plan,
                );
            } else {
                unreachable!();
            }
        }

        depth += 1;
        level = next_level;
    }
    floor_plan
}

fn alloc_half<D, C>(
    level: &Level<Forest<D, C>>,
    alloc: &mut Level<BTreeSet<Dir4>>,
    range: RangeInclusive<i8>,
) -> bool
where
    Tree<D, C>: RoomExt,
    D: std::fmt::Debug,
    C: std::fmt::Debug,
{
    eprintln!("<");
    #[derive(Debug)]
    struct Best {
        pos: i8,
        score: (isize, isize),
    }
    let mut best: Option<Best> = None;
    let mut seen_non_empty = false;
    for pos in range.clone() {
        let pos = pos as i8;
        if !level.is_empty(pos) {
            eprintln!("pos {} !empty", pos);
            seen_non_empty = true;
            if let Some(ref mut best) = &mut best {
                let score = level.score(pos);
                if score > best.score {
                    *best = Best { pos, score };
                }
            } else {
                let score = level.score(pos);
                best = Some(Best { pos, score });
            }
        } else {
            eprintln!("pos {} empty", pos);
            if let Some(Best { pos, .. }) = best {
                let mut pos2 = pos;
                while pos2 >= *range.start() && !level.is_empty(pos2) {
                    alloc.insert(pos2, Dir4::West);
                    alloc.insert(pos2, Dir4::North);
                    pos2 -= 1;
                }
                let mut pos2 = pos;
                while pos2 <= *range.end() && !level.is_empty(pos2) {
                    alloc.insert(pos2, Dir4::North);
                    alloc.insert(pos2, Dir4::East);
                    pos2 += 1;
                }
            }
            best = None;
        }
    }
    if let Some(Best { pos, .. }) = best {
        let mut pos2 = pos;
        while pos2 >= *range.start() && !level.is_empty(pos2) {
            alloc.insert(pos2, Dir4::West);
            alloc.insert(pos2, Dir4::North);
            pos2 -= 1;
        }
        let mut pos2 = pos;
        while pos2 <= *range.end() && !level.is_empty(pos2) {
            alloc.insert(pos2, Dir4::North);
            alloc.insert(pos2, Dir4::East);
            pos2 += 1;
        }
    }
    eprintln!(">");
    seen_non_empty
}