use bitvec::bitvec;
use bitvec::vec::BitVec;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref EMPTY_SET: HashSet<Index> = HashSet::new();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Index(usize);

pub struct Outline<A> {
    nodes: Vec<A>,
    deps: HashMap<Index, HashSet<Index>>,
    rev_deps: HashMap<Index, HashSet<Index>>,
}

impl<A> Outline<A> {
    pub fn new() -> Self {
        Outline {
            nodes: Vec::new(),
            deps: HashMap::new(),
            rev_deps: HashMap::new(),
        }
    }
    pub fn node(&mut self, action: A) -> Index {
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
    pub fn nodes(&self) -> &[A] {
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

impl<A> Default for Outline<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Clone> Outline<A> {
    pub fn get(&self, index: &Index) -> Option<A> {
        self.nodes.get(index.0).cloned()
    }
}

impl<A: Clone + Eq> Outline<A> {
    pub fn index(&self, action: A) -> Option<Index> {
        self.nodes.iter().position(|a| *a == action).map(Index)
    }
    pub fn action_sequence<R: Rng>(&self, rng: &mut R) -> Vec<A> {
        let weights = self.reachable_counts();
        let mut open = Vec::new();
        let mut closed = HashSet::new();
        for index in self.indices() {
            if self.deps(index).is_empty() {
                open.push(index);
            }
        }
        //let mut rng = StepRng::new(19, 13);
        let mut results = Vec::new();
        while !open.is_empty() {
            let index = *open
                .choose_weighted(rng, |index| weights.get(index).unwrap())
                .unwrap();
            open.retain(|&x| x != index);
            closed.insert(index);
            open.extend(
                self.rev_deps(index)
                    .iter()
                    .cloned()
                    .filter(|&src| self.deps(src).iter().all(|dest| closed.contains(dest))),
            );
            results.push(self.get(&index).unwrap());
        }
        results
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
