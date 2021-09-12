pub mod event_tree;
pub mod outline;
pub mod tunics;

use crate::event_tree::Tree;

fn main() {
    use crate::tunics::calc_join_weight;
    use crate::tunics::hide_chests;
    use crate::tunics::OutlineConf;
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(3);
    //let mut rng = rand::rngs::ThreadRng::default();

    let outline = OutlineConf {
        num_fairies: 0,
        num_cul_de_sacs: 0,
        num_small_keys: 0,
        //treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        treasures: [].iter().cloned().collect(),
    }
    .into_outline();
    let actions = outline.action_sequence(&mut rng);

    /*
    outline.show();
    for action in &actions {
        println!("{:?}", action);
    }
    */

    let mut tree = Tree::from_actions(&mut rng, 3, &actions, calc_join_weight);
    hide_chests(&mut rng, &mut tree);
    tree.show();

    let room = tree.room_tree();
    room.show();
}
