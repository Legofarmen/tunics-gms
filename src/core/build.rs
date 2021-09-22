use crate::core::feature::Command;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

lazy_static! {
    static ref EMPTY_SET: HashSet<Index> = HashSet::new();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Index(usize);

pub struct BuildPlan<E, T>
where
    E: Debug,
    T: Copy,
{
    steps: Vec<Command<E, T>>,
    outgoing: HashMap<Index, HashSet<Index>>, // (Source, Target)
    incoming: HashMap<Index, HashSet<Index>>, // (Target, Source)
}

impl<E, T> BuildPlan<E, T>
where
    E: Debug,
    T: Copy,
{
    pub fn new() -> Self {
        BuildPlan {
            steps: Vec::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
        }
    }
    pub fn step(&mut self, step: Command<E, T>) -> Index {
        let i = self.steps.len();
        self.steps.push(step);
        Index(i)
    }
    pub fn arc(&mut self, source: Index, dest: Index) {
        self.outgoing
            .entry(source)
            .or_insert_with(HashSet::new)
            .insert(dest);
        self.incoming
            .entry(dest)
            .or_insert_with(HashSet::new)
            .insert(source);
    }
    pub fn sorted(&self) -> Vec<Index> {
        fn visit(
            index: Index,
            outgoing: &HashMap<Index, HashSet<Index>>,
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
            if let Some(targets) = outgoing.get(&index) {
                for target in targets {
                    visit(*target, outgoing, permanent, temporary, result);
                }
            }
            temporary.remove(&index);
            permanent.insert(index);
            result.push(index);
        }
        let mut permanent = HashSet::new();
        let mut temporary = HashSet::new();
        let mut result = Vec::new();
        while let Some(index) = (0..self.steps.len())
            .map(Index)
            .find(|step| !permanent.contains(step))
        {
            visit(
                index,
                &self.outgoing,
                &mut permanent,
                &mut temporary,
                &mut result,
            );
        }
        result
    }
    pub fn indices(&self) -> impl Iterator<Item = Index> {
        (0..self.steps.len()).map(Index)
    }
    pub fn steps(&self) -> &[Command<E, T>] {
        self.steps.as_slice()
    }
    pub fn outgoing(&self, source: Index) -> &HashSet<Index> {
        self.outgoing.get(&source).unwrap_or(&EMPTY_SET)
    }
    pub fn incoming(&self, source: Index) -> &HashSet<Index> {
        self.incoming.get(&source).unwrap_or(&EMPTY_SET)
    }
    pub fn reachable_counts(&self) -> HashMap<Index, usize> {
        let mut masks: HashMap<Index, BitVec> = HashMap::new();
        let len = self.steps.len();
        for index in self.sorted().iter().rev() {
            let mut bv = masks.remove(index).unwrap_or_else(|| bitvec![0; len]);
            bv.insert(index.0, true);
            for src in self.incoming(*index) {
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

impl<E, T> BuildPlan<E, T>
where
    E: Clone + Debug,
    T: Copy,
{
    pub fn get(&self, index: &Index) -> Option<Command<E, T>> {
        self.steps.get(index.0).cloned()
    }
}

impl<E, T> Default for BuildPlan<E, T>
where
    E: Debug,
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildSequence<'a, E, T, F>
where
    E: Debug,
    F: FnMut(&[Index]) -> Index,
    T: Copy,
{
    traversal_selector: F,
    build_plan: &'a BuildPlan<E, T>,
    open: Vec<Index>,
    closed: HashSet<Index>,
}

impl<'a, E, T, F> Iterator for BuildSequence<'a, E, T, F>
where
    E: Clone + Debug,
    F: FnMut(&[Index]) -> Index,
    T: Copy,
{
    type Item = Command<E, T>;

    fn next(&mut self) -> Option<Command<E, T>> {
        if self.open.is_empty() {
            return None;
        }

        let BuildSequence {
            traversal_selector,
            open,
            closed,
            build_plan,
            ..
        } = self;

        let index = traversal_selector(open);
        open.retain(|&x| x != index);
        closed.insert(index);
        open.extend(build_plan.incoming(index).iter().cloned().filter(|&src| {
            build_plan
                .outgoing(src)
                .iter()
                .all(|dest| closed.contains(dest))
        }));

        build_plan.get(&index)
    }
}

impl<E, T> BuildPlan<E, T>
where
    E: Debug + Eq,
    T: Copy + Eq,
{
    pub fn build_sequence<'a, F>(&'a self, traversal_selector: F) -> BuildSequence<'a, E, T, F>
    where
        F: FnMut(&[Index]) -> Index,
    {
        let mut open = Vec::new();
        for index in self.indices() {
            if self.outgoing(index).is_empty() {
                open.push(index);
            }
        }
        BuildSequence {
            build_plan: self,
            traversal_selector,
            open,
            closed: HashSet::new(),
        }
    }

    pub fn index(&self, step: Command<E, T>) -> Option<Index> {
        self.steps.iter().position(|a| *a == step).map(Index)
    }
}

impl<A: Debug, T: Copy + Debug> BuildPlan<A, T> {
    pub fn show(&self) {
        for index in self.sorted() {
            let mut outgoing: Vec<_> = self
                .outgoing
                .get(&index)
                .iter()
                .flat_map(|indices| indices.iter())
                .map(|index| format!("{}", index.0))
                .collect();
            outgoing.sort();
            println!(
                "{}. {:?} ({})",
                index.0,
                self.steps[index.0],
                outgoing.join(", ")
            );
        }
    }
}
