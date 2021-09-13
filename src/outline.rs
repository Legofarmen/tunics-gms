use crate::event_tree::Action;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

lazy_static! {
    static ref EMPTY_SET: HashSet<Index> = HashSet::new();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Index(usize);

pub struct Outline<E>
where
    E: std::fmt::Debug,
{
    nodes: Vec<Action<E>>,
    deps: HashMap<Index, HashSet<Index>>,
    rev_deps: HashMap<Index, HashSet<Index>>,
}

impl<E> Outline<E>
where
    E: std::fmt::Debug,
{
    pub fn new() -> Self {
        Outline {
            nodes: Vec::new(),
            deps: HashMap::new(),
            rev_deps: HashMap::new(),
        }
    }
    pub fn node(&mut self, action: Action<E>) -> Index {
        let i = self.nodes.len();
        self.nodes.push(action);
        Index(i)
    }
    pub fn dep(&mut self, source: Index, dest: Index) {
        self.deps
            .entry(source)
            .or_insert_with(HashSet::new)
            .insert(dest);
        self.rev_deps
            .entry(dest)
            .or_insert_with(HashSet::new)
            .insert(source);
    }
    pub fn sorted(&self) -> Vec<Index> {
        fn visit(
            index: Index,
            deps: &HashMap<Index, HashSet<Index>>,
            permanent: &mut HashSet<Index>,
            temporary: &mut HashSet<Index>,
            result: &mut Vec<Index>,
        ) {
            if permanent.contains(&index) {
                return;
            }
            if temporary.contains(&index) {
                panic!("not a DAG");
            }
            temporary.insert(index);
            if let Some(targets) = deps.get(&index) {
                for target in targets {
                    visit(*target, deps, permanent, temporary, result);
                }
            }
            temporary.remove(&index);
            permanent.insert(index);
            result.push(index);
        }
        let mut permanent = HashSet::new();
        let mut temporary = HashSet::new();
        let mut result = Vec::new();
        while let Some(index) = (0..self.nodes.len())
            .map(Index)
            .find(|node| !permanent.contains(node))
        {
            visit(
                index,
                &self.deps,
                &mut permanent,
                &mut temporary,
                &mut result,
            );
        }
        result
    }
    pub fn indices(&self) -> impl Iterator<Item = Index> {
        (0..self.nodes.len()).map(Index)
    }
    pub fn nodes(&self) -> &[Action<E>] {
        self.nodes.as_slice()
    }
    pub fn deps(&self, source: Index) -> &HashSet<Index> {
        self.deps.get(&source).unwrap_or(&EMPTY_SET)
    }
    pub fn rev_deps(&self, source: Index) -> &HashSet<Index> {
        self.rev_deps.get(&source).unwrap_or(&EMPTY_SET)
    }
    pub fn reachable_counts(&self) -> HashMap<Index, usize> {
        let mut masks: HashMap<Index, BitVec> = HashMap::new();
        let len = self.nodes.len();
        for index in self.sorted().iter().rev() {
            let mut bv = masks.remove(index).unwrap_or_else(|| bitvec![0; len]);
            bv.insert(index.0, true);
            for src in self.rev_deps(*index) {
                *bv |= masks.get(src).unwrap().clone();
            }
            masks.insert(*index, bv);
        }
        masks
            .into_iter()
            .map(|(index, mask)| (index, mask.count_ones()))
            .collect()
    }
}

impl<E> Default for Outline<E>
where
    E: std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Outline<E>
where
    E: Clone + std::fmt::Debug,
{
    pub fn get(&self, index: &Index) -> Option<Action<E>> {
        self.nodes.get(index.0).cloned()
    }
}

pub struct ActionIter<'a, E, R>
where
    E: Debug,
    R: Rng,
{
    rng: &'a mut R,
    outline: &'a Outline<E>,
    weights: HashMap<Index, usize>,
    open: Vec<Index>,
    closed: HashSet<Index>,
}

impl<'a, E, R> Iterator for ActionIter<'a, E, R>
where
    E: Clone + Debug,
    R: Rng,
{
    type Item = Action<E>;
    fn next(&mut self) -> Option<Action<E>> {
        if self.open.is_empty() {
            return None;
        }

        let ActionIter {
            weights,
            open,
            rng,
            closed,
            outline,
            ..
        } = self;

        let index = *open
            .choose_weighted(rng, |index| weights.get(index).unwrap())
            .unwrap();
        open.retain(|&x| x != index);
        closed.insert(index);
        open.extend(
            outline
                .rev_deps(index)
                .iter()
                .cloned()
                .filter(|&src| outline.deps(src).iter().all(|dest| closed.contains(dest))),
        );

        outline.get(&index)
    }
}

impl<E> Outline<E>
where
    E: Clone + Debug + Eq,
{
    pub fn action_iter<'a, R>(&'a self, rng: &'a mut R) -> ActionIter<'a, E, R>
    where
        R: Rng,
    {
        let mut open = Vec::new();
        for index in self.indices() {
            if self.deps(index).is_empty() {
                open.push(index);
            }
        }
        ActionIter {
            rng,
            outline: &self,
            weights: self.reachable_counts(),
            open,
            closed: HashSet::new(),
        }
    }

    pub fn index(&self, action: Action<E>) -> Option<Index> {
        self.nodes.iter().position(|a| *a == action).map(Index)
    }
}

impl<A: std::fmt::Debug> Outline<A> {
    pub fn show(&self) {
        for index in self.sorted() {
            let mut deps: Vec<_> = self
                .deps
                .get(&index)
                .iter()
                .flat_map(|indices| indices.iter())
                .map(|index| format!("{}", index.0))
                .collect();
            deps.sort();
            println!(
                "{}. {:?} ({})",
                index.0,
                self.nodes[index.0],
                deps.join(", ")
            );
        }
    }
}
