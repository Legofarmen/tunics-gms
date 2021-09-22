pub mod core;
pub mod tunics;

use rand::Rng;
use rand::SeedableRng;

fn split_rng<R: Rng>(rng: &mut R) -> impl Rng {
    rand::rngs::StdRng::seed_from_u64(rng.gen())
}

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
    use crate::tunics::compact;
    use crate::tunics::get_traversal_selector;
    use crate::tunics::Config;
    use crate::tunics::Treasure;
    use rand::rngs::StdRng;
    use rand::rngs::ThreadRng;

    let seed = ThreadRng::default().gen();
    println!("{}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let rng2 = split_rng(&mut rng);

    let build_plan = BuildPlan::from(Config {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    })
    //.check(|build_plan| build_plan.show())
    ;

    let traversal_selector = get_traversal_selector(rng2, &build_plan);
    let _ = build_plan
        .build_sequence(traversal_selector)
        //.inspect(|step| println!("{:?}", step))
        .fold(FeaturePlan::default(), |feature_plan, step| {
            let feature_plan = step.apply(&mut rng, feature_plan);
            compact(&mut rng, 3, feature_plan)
        })
        //.check(|feature_plan| feature_plan.show())
        .into_room()
        .check(|room| room.show());
}
