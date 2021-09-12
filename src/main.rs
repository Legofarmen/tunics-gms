pub mod event_tree;
pub mod outline;
pub mod tunics;

fn main() {
    use crate::event_tree::Tree;
    use crate::tunics::calc_join_weight;
    use crate::tunics::OutlineConf;
    use crate::tunics::Treasure;
    use rand::SeedableRng;

    let mut rng = rand::rngs::ThreadRng::default();
    //let mut rng = rand::rngs::StdRng::seed_from_u64(3);

    let outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
    }
    .into_outline();
    let actions = outline.action_sequence(&mut rng);

    /*
    outline.show();
    for action in &actions {
        println!("{:?}", action);
    }
    */

    let tree = Tree::from_actions(&mut rng, 3, &actions, calc_join_weight);
    tree.show();
}
