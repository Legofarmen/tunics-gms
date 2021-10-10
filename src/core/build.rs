use crate::core::feature::Op;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

lazy_static! {
    static ref EMPTY_SET: BTreeSet<Index> = BTreeSet::new();
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Index(usize);

pub struct BuildPlan<E>
where
    E: Debug,
{
    steps: Vec<Op<E>>,
    outgoing: HashMap<Index, BTreeSet<Index>>, // (Source, Target)
    incoming: HashMap<Index, BTreeSet<Index>>, // (Target, Source)
}

impl<E> BuildPlan<E>
where
    E: Debug,
{
    pub fn new() -> Self {
        BuildPlan {
            steps: Vec::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
        }
    }
    pub fn vertex(&mut self, vertex: Op<E>) -> Index {
        let i = self.steps.len();
        self.steps.push(vertex);
        Index(i)
    }
    pub fn arc(&mut self, source: Index, dest: Index) {
        self.outgoing
            .entry(source)
            .or_insert_with(BTreeSet::new)
            .insert(dest);
        self.incoming
            .entry(dest)
            .or_insert_with(BTreeSet::new)
            .insert(source);
    }
    pub fn sorted(&self) -> Vec<Index> {
        fn visit(
            index: Index,
            outgoing: &HashMap<Index, BTreeSet<Index>>,
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
            .find(|vertex| !permanent.contains(vertex))
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
    pub fn steps(&self) -> &[Op<E>] {
        self.steps.as_slice()
    }
    pub fn outgoing(&self, source: Index) -> &BTreeSet<Index> {
        self.outgoing.get(&source).unwrap_or(&EMPTY_SET)
    }
    pub fn incoming(&self, source: Index) -> &BTreeSet<Index> {
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

impl<E> BuildPlan<E>
where
    E: Clone + Debug,
{
    pub fn get(&self, index: &Index) -> Option<Op<E>> {
        self.steps.get(index.0).cloned()
    }
}

impl<E> Default for BuildPlan<E>
where
    E: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildSequence<'a, E, F>
where
    E: Debug,
    F: FnMut(&[Index]) -> Index,
{
    traversal_selector: F,
    build_plan: &'a BuildPlan<E>,
    open: Vec<Index>,
    closed: HashSet<Index>,
}

impl<'a, E, F> Iterator for BuildSequence<'a, E, F>
where
    E: Clone + Debug,
    F: FnMut(&[Index]) -> Index,
{
    type Item = Op<E>;

    fn next(&mut self) -> Option<Op<E>> {
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

        // BuildPlan.incoming.iter() needs to be deterministic
        open.extend(build_plan.incoming(index).iter().cloned().filter(|&src| {
            build_plan
                .outgoing(src)
                .iter()
                .all(|dest| closed.contains(dest))
        }));

        build_plan.get(&index)
    }
}

impl<E> BuildPlan<E>
where
    E: Debug + Eq,
{
    pub fn build_sequence<F>(&self, traversal_selector: F) -> BuildSequence<E, F>
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

    pub fn index(&self, vertex: Op<E>) -> Option<Index> {
        self.steps.iter().position(|a| *a == vertex).map(Index)
    }
}

impl<A: Debug> BuildPlan<A> {
    pub fn show<M: std::fmt::Display>(&self, metadata: M, seed: u64) {
        println!("digraph {{");
        println!("  labelloc=\"t\";");
        println!(
            "  label=<<b>Build plan</b><br/>{}<br/>seed: {}>;",
            metadata, seed
        );
        for (i, op) in self.steps.iter().enumerate() {
            println!("  step{} [label=\"{:?}\"];", i, op);
        }
        for (source, targets) in &self.outgoing {
            for target in targets {
                println!("  step{} -> step{};", source.0, target.0);
            }
        }
        println!("}}");
    }
}
