mod outline;

fn main() {
    use outline::Outline;
    use outline::OutlineConf;
    use outline::Treasure;

    let mut rng = rand::rngs::mock::StepRng::new(1, 3);
    //let mut rng = rand::rngs::ThreadRng::default();

    let outline: Outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
    }
    .into();
    let actions = outline.action_sequence(&mut rng);

    //outline.show();
    for action in actions {
        println!("{:?}", action);
    }
}
