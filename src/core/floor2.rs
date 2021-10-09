use crate::core::floor::FloorPlan;
use crate::core::room::Forest;
use crate::core::room::RoomExt;
use crate::core::room::Tree;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

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
    dir: Dir3,
    floor_plan: &mut FloorPlan,
) where
    Tree<D, C>: RoomExt,
{
    let delta = match dir {
        Dir3::North => 0,
        Dir3::East => 1,
        Dir3::West => -1,
    };
    let forest = match forest.pop_tree() {
        Ok((door, contents, forest)) => {
            floor_plan.add_door((x, y, dir), door);
            floor_plan.add_room((x, y), contents);
            forest
        }
        Err(forest) => forest,
    };
    next_level.set(x + delta, forest);
}

pub fn doit<D, C, T: Ord>()
where
    Tree<D, C>: RoomExt,
{
    let depth = 0;
    let mut floor_plan = FloorPlan::default();

    let mut level = Level::<Forest<D, C>>::new_forests(depth);

    let mut seen_non_empty = true;
    while seen_non_empty {
        let mut alloc = Level::new_sets(depth);

        seen_non_empty =
            alloc_half(&level, &mut alloc, depth..=0) | alloc_half(&level, &mut alloc, 0..=-depth);

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

        level = next_level;
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Dir3 {
    North,
    East,
    West,
}

impl Dir3 {
    fn delta(self) -> i8 {}
}

fn alloc_half<D, C>(
    level: &Level<Forest<D, C>>,
    alloc: &mut Level<BTreeSet<Dir3>>,
    range: RangeInclusive<i8>,
) -> bool
where
    Tree<D, C>: RoomExt,
{
    struct Best {
        pos: i8,
        score: (isize, isize),
    }
    let mut best: Option<Best> = None;
    let mut seen_non_empty = false;
    for pos in range {
        let pos = pos as i8;
        if !level.is_empty(pos) {
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
            if let Some(Best { pos, .. }) = best {
                let mut pos2 = pos;
                while !level.is_empty(pos2) {
                    alloc.insert(pos2, Dir3::West);
                    alloc.insert(pos2, Dir3::North);
                    pos2 -= 1;
                }
                let mut pos2 = pos;
                while !level.is_empty(pos2) {
                    alloc.insert(pos2, Dir3::North);
                    alloc.insert(pos2, Dir3::East);
                    pos2 += 1;
                }
            }
            best = None;
        }
    }
    seen_non_empty
}
