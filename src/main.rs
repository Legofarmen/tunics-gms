pub mod event_tree;
pub mod outline;
pub mod tunics;

use rand::Rng;
use rand::SeedableRng;

fn split_rng<R: Rng>(rng: &mut R) -> impl Rng {
    rand::rngs::StdRng::seed_from_u64(rng.gen())
}

fn main() {
    use crate::event_tree::Tree;
    use crate::outline::Outline;
    use crate::tunics::hide_chests;
    use crate::tunics::Config;
    use crate::tunics::MyCompacter;
    use crate::tunics::Treasure;
    use rand::rngs::StdRng;
    use rand::rngs::ThreadRng;

    let seed = ThreadRng::default().gen();
    println!("{}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rng2 = split_rng(&mut rng);

    let outline = Outline::from(Config {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    });

    let actions = outline.action_iter(&mut rng2);
    let mut tree = Tree::from_actions(&mut rng, actions, &MyCompacter { max_heads: 3 });

    hide_chests(&mut rng, &mut tree);

    let room = tree.into_room();

    /*
    outline.show();
    for action in &actions {
        println!("{:?}", action);
    }
    tree.show();
    */

    room.show();
}
