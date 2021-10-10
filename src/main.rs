pub mod core;
pub mod tunics;

use crate::core::build::BuildPlan;
use crate::core::feature::FeaturePlan;
use crate::core::feature::Op;
use crate::core::floor::from_forest;
use crate::core::floor::FloorPlan;
use crate::core::room::Tree as RoomTree;
use crate::tunics::AugFeature;
use crate::tunics::Config;
use crate::tunics::Contents;
use crate::tunics::Door;
use crate::tunics::Feature;
use crate::tunics::Item;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// PRNG seed
    #[structopt(long)]
    seed: Option<u64>,

    #[structopt(long, default_value)]
    fairies: usize,

    #[structopt(long, default_value)]
    cul_de_sacs: usize,

    #[structopt(long, default_value)]
    traps: usize,

    #[structopt(long, default_value)]
    small_keys: usize,

    #[structopt(long, default_value = "random")]
    items: Vec<OptItem>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    BuildPlan,
    BuildSequence,
    FeaturePlan1,
    FeaturePlan2,
    RoomPlan,
    FloorPlan,
}

enum OptItem {
    Item(Item),
    Random,
}

impl std::str::FromStr for OptItem {
    type Err = <Item as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "random" {
            Ok(OptItem::Random)
        } else {
            Item::from_str(s).map(OptItem::Item)
        }
    }
}

fn build_plan(seed: u64, config: &Config) -> (impl Rng, BuildPlan<AugFeature>) {
    let rng = StdRng::seed_from_u64(seed);
    (rng, BuildPlan::from(config.clone()))
}

fn build_sequence(seed: u64, config: &Config) -> (impl Rng, Vec<Op<AugFeature>>) {
    use crate::tunics::get_traversal_selector;

    let (mut rng, build_plan) = build_plan(seed, config);

    let rng1 = StdRng::seed_from_u64(rng.gen());

    let traversal_selector = get_traversal_selector(rng1, &build_plan);
    let build_sequence = build_plan.build_sequence(traversal_selector);
    (rng, build_sequence.collect())
}

fn feature_plan1(seed: u64, config: &Config) -> (impl Rng, FeaturePlan<AugFeature>) {
    use crate::tunics::get_join_selector;
    use crate::tunics::get_prepend_selector;

    let (mut rng, build_sequence) = build_sequence(seed, config);

    let rng2 = StdRng::seed_from_u64(rng.gen());
    let rng3 = StdRng::seed_from_u64(rng.gen());

    let prepend_selector = get_prepend_selector(rng2);
    let join_selector = get_join_selector(rng3);
    (
        rng,
        FeaturePlan::from_steps(join_selector, prepend_selector, build_sequence),
    )
}

fn feature_plan2(seed: u64, config: &Config) -> (impl Rng, FeaturePlan<Feature>) {
    use crate::tunics::lower;

    let (mut rng, feature_plan) = feature_plan1(seed, config);
    let mut rng4 = StdRng::seed_from_u64(rng.gen());

    (rng, lower(&mut rng4, feature_plan))
}

fn room_plan(seed: u64, config: &Config) -> (impl Rng, RoomTree<Door, Contents>) {
    let (rng, feature_plan) = feature_plan2(seed, config);
    (rng, RoomTree::from_feature_plan(feature_plan))
}

fn floor_plan(seed: u64, config: &Config) -> (impl Rng, FloorPlan) {
    let (rng, room_plan) = room_plan(seed, config);
    (rng, from_forest(room_plan))
}

fn show_vec<T: std::fmt::Debug, M: std::fmt::Display>(
    sequence: Vec<T>,
    title: &str,
    metadata: M,
    seed: u64,
) {
    println!("digraph {{");
    println!("  labelloc=\"t\";");
    println!(
        "  label=<<b>{}</b><br/>{}<br/>seed: {}>;",
        title, metadata, seed,
    );
    for (i, elem) in sequence.iter().enumerate() {
        println!("  elem{} [label=\"{:?}\"];", i, elem);
        if i > 0 {
            println!("  elem{} -> elem{};", i - 1, i);
        }
    }
    println!("}}");
}

fn main() {
    use rand::rngs::ThreadRng;

    let opt = Opt::from_args();
    let seed = opt.seed.unwrap_or_else(|| ThreadRng::default().gen());

    let mut rng5 = StdRng::seed_from_u64(seed);
    let mut all_items = Item::all();
    let items = opt
        .items
        .iter()
        .map(|item| match item {
            OptItem::Item(item) => *item,
            OptItem::Random => {
                let i = rng5.gen_range(0..all_items.len());
                all_items.swap_remove(i)
            }
        })
        .collect();
    eprintln!("seed {}", seed);
    eprintln!("items {:?}", items);

    let config = Config {
        num_small_keys: opt.small_keys,
        num_fairies: opt.fairies,
        num_cul_de_sacs: opt.cul_de_sacs,
        num_traps: opt.traps,
        items,
    };

    match opt.cmd {
        Command::BuildPlan => {
            build_plan(seed, &config).1.show(config, seed);
        }
        Command::BuildSequence => {
            show_vec(
                build_sequence(seed, &config).1,
                "Build sequence",
                config,
                seed,
            );
        }
        Command::FeaturePlan1 => {
            feature_plan1(seed, &config)
                .1
                .show("Feature plan 1", config, seed);
        }
        Command::FeaturePlan2 => {
            feature_plan2(seed, &config)
                .1
                .show("Feature plan 2", config, seed);
        }
        Command::RoomPlan => {
            room_plan(seed, &config).1.show(config, seed);
        }
        Command::FloorPlan => {
            floor_plan(seed, &config).1.show(config, seed);
        }
    };
}
