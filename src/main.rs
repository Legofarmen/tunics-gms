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
    println!("{}", seed);

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
        //.inspect(|step| println!("{:?}", step))
        ;
    let feature_plan = FeaturePlan::from_steps(join_selector, prepend_selector, build_sequence)
        //.check(|feature_plan| feature_plan.show())
        ;
    Room::from_feature_plan(hide_chests(&mut rng4, feature_plan)).check(Room::show);
}
