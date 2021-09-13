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
    use crate::tunics::calc_join_weight;
    use crate::tunics::hide_chests;
    use crate::tunics::OutlineConf;
    use crate::tunics::Treasure;
    //let mut rng = rand::rngs::StdRng::seed_from_u64(3);
    let mut rng = rand::rngs::ThreadRng::default();

    let outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        //treasures: [].iter().cloned().collect(),
    }
    .into_outline();
    let mut rng2 = split_rng(&mut rng);
    let actions = outline.action_iter(&mut rng2);

    let mut tree = Tree::from_actions(&mut rng, actions, 3, calc_join_weight);
    hide_chests(&mut rng, &mut tree);

    /*
    outline.show();
    for action in &actions {
        println!("{:?}", action);
    }
    tree.show();
    */

    let room = tree.room_tree();
    room.show();
}
