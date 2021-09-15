pub mod feature_tree;
pub mod requirements;
pub mod tunics;

use rand::Rng;
use rand::SeedableRng;

fn split_rng<R: Rng>(rng: &mut R) -> impl Rng {
    rand::rngs::StdRng::seed_from_u64(rng.gen())
}

fn main() {
    use crate::feature_tree::FeatureTree;
    use crate::requirements::Requirements;
    use crate::tunics::compact;
    use crate::tunics::Config;
    use crate::tunics::Treasure;
    use rand::rngs::StdRng;
    use rand::rngs::ThreadRng;

    let seed = ThreadRng::default().gen();
    println!("{}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rng2 = split_rng(&mut rng);

    let requirements = Requirements::from(Config {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    });
    let tree = requirements
        .action_iter(&mut rng2)
        .fold(FeatureTree::default(), |tree, action| {
            let tree = action.apply(&mut rng, tree);
            compact(&mut rng, 3, tree)
        });
    let room = tree.into_room();

    /*
    requirements.show();
    for action in &actions {
        println!("{:?}", action);
    }
    tree.show();
    */
    room.show();
}
