pub mod core;
pub mod tunics;

use crate::core::build::BuildPlan;
use crate::core::feature::FeaturePlan;
use crate::core::floor::FloorPlan;
use crate::core::floor2::from_forest;
use crate::core::room::Tree as RoomTree;
use crate::tunics::AugFeature;
use crate::tunics::Contents;
use crate::tunics::Door;
use crate::tunics::Feature;
use crate::tunics::Item;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// PRNG seed
    seed: Option<u64>,

    #[structopt(long, default_value)]
    fairies: usize,

    #[structopt(long, default_value)]
    cul_de_sacs: usize,

    #[structopt(long, default_value)]
    traps: usize,

    #[structopt(long, default_value)]
    small_keys: usize,

    #[structopt(long)]
    items: Option<Vec<Item>>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    BuildPlan,
    FeaturePlan1,
    FeaturePlan2,
    RoomPlan,
    FloorPlan,
}

fn build_plan(seed: u64, opt: Opt) -> (impl Rng, BuildPlan<AugFeature>) {
    use crate::tunics::gen_treasure_set;
    use crate::tunics::Config;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(seed);
    let items = opt
        .items
        .map(|items| items.into_iter().collect())
        .unwrap_or_else(|| gen_treasure_set(&mut rng, 1));
    (
        rng,
        BuildPlan::from(Config {
            num_fairies: opt.fairies,
            num_cul_de_sacs: opt.cul_de_sacs,
            num_small_keys: opt.small_keys,
            num_traps: opt.traps,
            items,
        }),
    )
}

fn feature_plan1(seed: u64, opt: Opt) -> (impl Rng, FeaturePlan<AugFeature>) {
    use crate::tunics::get_join_selector;
    use crate::tunics::get_prepend_selector;
    use crate::tunics::get_traversal_selector;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let (mut rng, build_plan) = build_plan(seed, opt);

    let rng1 = StdRng::seed_from_u64(rng.gen());
    let rng2 = StdRng::seed_from_u64(rng.gen());
    let rng3 = StdRng::seed_from_u64(rng.gen());

    let traversal_selector = get_traversal_selector(rng1, &build_plan);
    let prepend_selector = get_prepend_selector(rng2);
    let join_selector = get_join_selector(rng3);
    let build_sequence = build_plan
        .build_sequence(traversal_selector)
        //.inspect(|step| eprintln!("{:?}", step))
        //;
        ;
    (
        rng,
        FeaturePlan::from_steps(join_selector, prepend_selector, build_sequence),
    )
}

fn feature_plan2(seed: u64, opt: Opt) -> (impl Rng, FeaturePlan<Feature>) {
    use crate::tunics::lower;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let (mut rng, feature_plan) = feature_plan1(seed, opt);
    let mut rng4 = StdRng::seed_from_u64(rng.gen());

    (rng, lower(&mut rng4, feature_plan))
}

fn room_plan(seed: u64, opt: Opt) -> (impl Rng, RoomTree<Door, Contents>) {
    let (rng, feature_plan) = feature_plan2(seed, opt);
    (rng, RoomTree::from_feature_plan(feature_plan))
}

fn floor_plan(seed: u64, opt: Opt) -> (impl Rng, FloorPlan) {
    let (rng, room_plan) = room_plan(seed, opt);
    (rng, from_forest(room_plan))
}

fn main() {
    use rand::rngs::ThreadRng;

    let opt = Opt::from_args();
    let seed = opt.seed.unwrap_or_else(|| ThreadRng::default().gen());
    eprintln!("{}", seed);

    match opt.cmd {
        Command::BuildPlan => {
            build_plan(seed, opt).1.show();
        }
        Command::FeaturePlan1 => {
            feature_plan1(seed, opt).1.show();
        }
        Command::FeaturePlan2 => {
            feature_plan2(seed, opt).1.show();
        }
        Command::RoomPlan => {
            room_plan(seed, opt).1.show();
        }
        Command::FloorPlan => {
            floor_plan(seed, opt).1.show();
        }
    };
}
