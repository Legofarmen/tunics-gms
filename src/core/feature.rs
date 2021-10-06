use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub enum FeaturePlan<F> {
    Feature(F, Box<FeaturePlan<F>>),
    Branch(Vec<FeaturePlan<F>>),
}

impl<F> fmt::Debug for FeaturePlan<F>
where
    F: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FeaturePlan::Feature(feature, next) => write!(f, "{:?} : {:?}", feature, next),
            FeaturePlan::Branch(nodes) => nodes.fmt(f),
        }
    }
}

impl<F> FeaturePlan<F> {
    pub fn new() -> Self {
        FeaturePlan::Branch(Vec::new())
    }

    pub fn new_feature(feature: F) -> Self {
        FeaturePlan::new().prepended(feature)
    }

    pub fn prepended(self, feature: F) -> Self {
        FeaturePlan::Feature(feature, Box::new(self))
    }

    pub fn find_feature_depth<'a, P>(&'a self, predicate: &P) -> Option<usize>
    where
        P: Fn(&'a F) -> bool,
        F: 'a,
    {
        match self {
            FeaturePlan::Feature(feature, next) => {
                if predicate(feature) {
                    Some(0)
                } else {
                    next.find_feature_depth(predicate).map(|depth| depth + 1)
                }
            }
            FeaturePlan::Branch(nodes) => nodes
                .iter()
                .map(|node| node.find_feature_depth(predicate).map(|depth| depth + 1))
                .fold(None, |acc, depth| match (acc, depth) {
                    (Some(acc), Some(depth)) => Some(acc.min(depth)),
                    (acc, depth) => acc.or(depth),
                }),
        }
    }

    pub fn max_depth(&self) -> usize {
        match self {
            FeaturePlan::Feature(_, next) => next.max_depth() + 1,
            FeaturePlan::Branch(nodes) => nodes
                .iter()
                .fold(1, |acc, node| acc.max(node.max_depth() + 1)),
        }
    }

    pub fn join(self, other: Self) -> Self {
        match (self, other) {
            (FeaturePlan::Branch(mut u), FeaturePlan::Branch(mut v)) => {
                u.append(&mut v);
                FeaturePlan::Branch(u)
            }
            (FeaturePlan::Branch(mut u), feature) => {
                u.push(feature);
                FeaturePlan::Branch(u)
            }
            (feature, FeaturePlan::Branch(mut u)) => {
                u.push(feature);
                FeaturePlan::Branch(u)
            }
            (feature1, feature2) => FeaturePlan::Branch(vec![feature1, feature2]),
        }
    }
}

impl<F> FeaturePlan<F>
where
    F: Clone,
{
    pub fn prepend(&mut self, feature: F) {
        *self = FeaturePlan::Feature(feature, Box::new(self.clone()));
    }
}

impl<F> FeaturePlan<F>
where
    F: Debug,
{
    pub fn show(&self) {
        fn visit<F>(plan: &FeaturePlan<F>, parent: usize, node: usize) -> usize
        where
            F: Debug,
        {
            match plan {
                FeaturePlan::Feature(feature, t) => {
                    println!("  n{} [label=\"{:?}\"];", node, feature);
                    let next = visit(t, node, node + 1);
                    println!("  n{} -- n{};", node, parent);
                    next
                }
                FeaturePlan::Branch(ts) => {
                    println!("  n{} [label=\"\"];", node);
                    println!("  n{} -- n{};", node, parent);
                    let mut next = node + 1;
                    for t in ts {
                        next = visit(t, node, next);
                    }
                    next
                }
            }
        }
        println!("graph {{");
        println!("  rankdir=BT;");
        visit(self, 0, 0);
        println!("}}");
    }
}

impl<F> FeaturePlan<F>
where
    F: Clone + Debug,
{
    pub fn from_steps<J, P, I>(
        mut join_selector: J,
        mut prepend_selector: P,
        steps: I,
    ) -> FeaturePlan<F>
    where
        J: FnMut(&[FeaturePlan<F>]) -> Option<(usize, usize)>,
        P: FnMut(&[FeaturePlan<F>]) -> usize,
        I: IntoIterator<Item = Op<F>>,
    {
        let mut feature_plan = FeaturePlan::new();

        for step in steps {
            feature_plan = match step {
                Op::New(feature) => {
                    let feature = FeaturePlan::new_feature(feature);
                    let mut nodes = match feature_plan {
                        FeaturePlan::Branch(mut nodes) => {
                            nodes.push(feature);
                            nodes
                        }
                        _ => vec![feature_plan, feature],
                    };
                    if nodes.len() > 2 {
                        if let Some((i, j)) = join_selector(&nodes) {
                            assert!(i != j);
                            let (m, n) = if i < j {
                                (nodes.swap_remove(j), nodes.swap_remove(i))
                            } else {
                                (nodes.swap_remove(i), nodes.swap_remove(j))
                            };
                            nodes.push(m.join(n));
                        }
                    }
                    FeaturePlan::Branch(nodes)
                }
                Op::PrependOne(feature) => match feature_plan {
                    FeaturePlan::Branch(mut nodes) => {
                        let i = prepend_selector(&nodes);
                        nodes.get_mut(i).unwrap().prepend(feature);
                        FeaturePlan::Branch(nodes)
                    }
                    _ => feature_plan.prepended(feature),
                },
                Op::PrependEach(feature) => match feature_plan {
                    FeaturePlan::Branch(nodes) => FeaturePlan::Branch(
                        nodes
                            .into_iter()
                            .map(|node| node.prepended(feature.clone()))
                            .collect(),
                    ),
                    _ => feature_plan.prepended(feature),
                },
                Op::PrependGrouped(feature) => feature_plan.prepended(feature),
            }
        }
        feature_plan
    }
}

impl<F> Default for FeaturePlan<F>
where
    F: Debug,
{
    fn default() -> Self {
        FeaturePlan::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op<F>
where
    F: Debug,
{
    New(F),
    PrependOne(F),
    PrependEach(F),
    PrependGrouped(F),
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn leaf() -> FeaturePlan<()> {
        FeaturePlan::Branch(Vec::new())
    }

    pub fn feature(feature_plan: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Feature((), Box::new(feature_plan))
    }

    pub fn branch1(node: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Branch(vec![node])
    }

    pub fn branch2(node1: FeaturePlan<()>, node2: FeaturePlan<()>) -> FeaturePlan<()> {
        FeaturePlan::Branch(vec![node1, node2])
    }

    #[test]
    fn test_max_depth() {
        assert_eq!(1, leaf().max_depth());
        assert_eq!(2, branch1(leaf()).max_depth());
        assert_eq!(2, branch2(leaf(), leaf()).max_depth());
        assert_eq!(3, branch1(branch1(leaf())).max_depth());
        assert_eq!(2, feature(leaf()).max_depth());
        assert_eq!(3, feature(feature(leaf())).max_depth());

        assert_eq!(
            7,
            branch2(
                feature(leaf()),
                feature(branch2(
                    feature(feature(feature(leaf()))),
                    feature(feature(feature(leaf()))),
                )),
            )
            .max_depth()
        );
    }
}
