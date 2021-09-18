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
    use crate::tunics::Config;
    use crate::tunics::Treasure;
    use rand::rngs::StdRng;
    use rand::rngs::ThreadRng;

    let seed = ThreadRng::default().gen();
    println!("{}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rng2 = split_rng(&mut rng);

    let _ = BuildPlan::from(Config {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    })
    //.check(|requirements| requirements.show())
    .action_iter(&mut rng2)
    //.inspect(|action| println!("{:?}", action))
    .fold(FeaturePlan::default(), |tree, action| {
        let tree = action.apply(&mut rng, tree);
        compact(&mut rng, 3, tree)
    })
    //.check(|tree| tree.show())
    .into_room()
    .check(|room| room.show());
}
