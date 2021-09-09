mod outline;

fn main() {
    use outline::Outline;
    use outline::OutlineConf;
    use outline::Treasure;

    let outline: Outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
    }
    .into();
    outline.show();
}
