use crate::core::room::Forest;
use crate::core::room::RoomExt;
use crate::core::room::Tree;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

pub struct Level<T> {
    depth: isize,
    cells: Vec<T>,
}

impl<D, C> Level<Forest<D, C>> {
    pub fn new_forests(depth: isize) -> Self {
        let width = 2 * depth as usize + 1;
        let mut forests = Vec::with_capacity(width);
        forests.resize_with(width, || Forest::new());
        Level {
            depth,
            cells: forests,
        }
    }
    pub fn is_empty(&self, pos: isize) -> bool {
        self.cells[(self.depth + pos) as usize].is_empty()
    }
}
impl<D, C> Level<Forest<D, C>>
where
    Tree<D, C>: RoomExt,
{
    pub fn score(&self, pos: isize) -> (isize, isize) {
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
    pub fn new_sets(depth: isize) -> Self {
        let width = 2 * depth as usize + 1;
        let mut sets = Vec::with_capacity(width);
        sets.resize_with(width, || BTreeSet::new());
        Level { depth, cells: sets }
    }
    pub fn insert(&mut self, pos: isize, elem: T) {
        self.cells[(self.depth + pos) as usize].insert(elem);
    }
}

pub fn doit<D, C, T: Ord>()
where
    Tree<D, C>: RoomExt,
{
    let depth = 0;

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
            if alloc.len() == 0 {
            } else if alloc.len() == 1 {
                let mut alloc = alloc.into_iter();
                let delta = alloc.next().unwrap().delta();
                next_level.cells[i + (delta + 1) as usize] = forest;
            } else if alloc.len() == 2 {
                let mut alloc = alloc.into_iter();
                let delta1 = alloc.next().unwrap().delta();
                let delta2 = alloc.next().unwrap().delta();
                let (forest1, forest2) = forest.split2();
                next_level.cells[i + (delta1 + 1) as usize] = forest1;
                next_level.cells[i + (delta2 + 1) as usize] = forest2;
            } else if alloc.len() == 3 {
                let mut alloc = alloc.into_iter();
                let delta1 = alloc.next().unwrap().delta();
                let delta2 = alloc.next().unwrap().delta();
                let delta3 = alloc.next().unwrap().delta();
                let (forest1, forest2, forest3) = forest.split3();
                next_level.cells[i + (delta1 + 1) as usize] = forest1;
                next_level.cells[i + (delta2 + 1) as usize] = forest2;
                next_level.cells[i + (delta3 + 1) as usize] = forest3;
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
    fn delta(self) -> isize {
        match self {
            Dir3::North => 0,
            Dir3::East => 1,
            Dir3::West => -1,
        }
    }
}

fn alloc_half<D, C>(
    level: &Level<Forest<D, C>>,
    alloc: &mut Level<BTreeSet<Dir3>>,
    range: RangeInclusive<isize>,
) -> bool
where
    Tree<D, C>: RoomExt,
{
    struct Best {
        pos: isize,
        score: (isize, isize),
    }
    let mut best: Option<Best> = None;
    let mut seen_non_empty = false;
    for pos in range {
        let pos = pos as isize;
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
