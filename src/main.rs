use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Obstacle {
    Entrance,
    WeakWall,
    VeryWeakWall,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Treasure {
    BigKey,
    BombsCounter,
    Map,
    Compass,
    SmallKey,
}

impl Treasure {
    fn get_obstacles(self) -> &'static [Obstacle] {
        match self {
            Treasure::BigKey => &[],
            Treasure::BombsCounter => &[Obstacle::WeakWall, Obstacle::VeryWeakWall],
            Treasure::Map => &[],
            Treasure::Compass => &[],
            Treasure::SmallKey => &[],
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Action {
    Boss,
    Chest(Treasure),
    BigChest(Treasure),
    LockedDoor,
    Fairy,
    CulDeSac,
    Obstacle(Obstacle),
    HideChests,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Index(usize);

struct Outline {
    nodes: Vec<Action>,
    deps: HashMap<Index, HashSet<Index>>,
}

impl Outline {
    fn new() -> Self {
        Outline {
            nodes: Vec::new(),
            deps: HashMap::new(),
        }
    }
    fn node(&mut self, action: Action) -> Index {
        let i = self.nodes.len();
        self.nodes.push(action);
        Index(i)
    }
    fn dep(&mut self, source: Index, dest: Index) {
        self.deps
            .entry(source)
            .or_insert_with(|| HashSet::new())
            .insert(dest);
    }
    fn index(&self, action: Action) -> Option<Index> {
        self.nodes.iter().position(|x| *x == action).map(Index)
    }
    fn sorted(&self) -> Vec<Index> {
        fn visit(
            index: Index,
            deps: &HashMap<Index, HashSet<Index>>,
            permanent: &mut HashSet<Index>,
            temporary: &mut HashSet<Index>,
            result: &mut Vec<Index>,
        ) {
            if permanent.contains(&index) {
                return;
            }
            if temporary.contains(&index) {
                panic!("not a DAG");
            }
            temporary.insert(index);
            if let Some(targets) = deps.get(&index) {
                for target in targets {
                    visit(*target, deps, permanent, temporary, result);
                }
            }
            temporary.remove(&index);
            permanent.insert(index);
            result.push(index);
        }
        let mut permanent = HashSet::new();
        let mut temporary = HashSet::new();
        let mut result = Vec::new();
        while let Some(index) = (0..self.nodes.len())
            .map(Index)
            .find(|node| !permanent.contains(node))
        {
            visit(
                index,
                &self.deps,
                &mut permanent,
                &mut temporary,
                &mut result,
            );
        }
        result
    }
    fn show(&self) {
        for index in self.sorted() {
            let mut deps: Vec<_> = self
                .deps
                .get(&index)
                .iter()
                .flat_map(|indices| indices.iter())
                .map(|index| format!("{}", index.0))
                .collect();
            deps.sort();
            println!(
                "{}. {:?} ({})",
                index.0,
                self.nodes[index.0],
                deps.join(", ")
            );
        }
    }
}

struct OutlineConf {
    num_small_keys: usize,
    num_fairies: usize,
    num_cul_de_sacs: usize,
    treasures: HashSet<Treasure>,
}

impl From<OutlineConf> for Outline {
    fn from(conf: OutlineConf) -> Self {
        let mut outline = Outline::new();
        let entrance = outline.node(Action::Obstacle(Obstacle::Entrance));

        for _ in 0..conf.num_cul_de_sacs {
            let cul_de_sac = outline.node(Action::CulDeSac);
            outline.dep(entrance, cul_de_sac);
        }

        for _ in 0..conf.num_fairies {
            let fairy = outline.node(Action::Fairy);
            outline.dep(entrance, fairy);
        }

        let boss = outline.node(Action::Boss);
        let big_key = outline.node(Action::Chest(Treasure::BigKey));
        outline.dep(big_key, boss);

        let hide_chests = outline.node(Action::HideChests);
        let compass = outline.node(Action::Chest(Treasure::Compass));
        outline.dep(hide_chests, boss);
        outline.dep(compass, hide_chests);
        outline.dep(entrance, compass);

        for treasure in &conf.treasures {
            let big_chest = outline.node(Action::BigChest(*treasure));
            outline.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = outline.node(Action::Obstacle(*obstacle));
                outline.dep(big_chest, obstacle);
                outline.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..conf.num_small_keys {
            let locked_door = outline.node(Action::LockedDoor);
            if let Some(last_locked_door) = last_locked_door {
                outline.dep(locked_door, last_locked_door);
            } else {
                outline.dep(locked_door, big_key);
            }
            if i == conf.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..conf.num_small_keys {
                    let small_key = outline.node(Action::Chest(Treasure::SmallKey));
                    if let Some(last_small_key) = last_small_key {
                        outline.dep(small_key, last_small_key);
                    } else {
                        outline.dep(small_key, locked_door);
                    }
                    if j == conf.num_small_keys - 1 {
                        outline.dep(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if conf.num_small_keys == 0 {
            outline.dep(entrance, big_key);
        }

        let map = outline.node(Action::Chest(Treasure::Map));
        if let Some(weak_wall) = outline.index(Action::Obstacle(Obstacle::WeakWall)) {
            let very_weak_wall = outline
                .index(Action::Obstacle(Obstacle::VeryWeakWall))
                .unwrap();
            outline.dep(very_weak_wall, weak_wall);
            outline.dep(map, very_weak_wall);
        } else {
            outline.dep(map, boss);
        }
        outline.dep(entrance, map);

        outline
    }
}

fn main() {
    let outline: Outline = OutlineConf {
        num_fairies: 1,
        num_cul_de_sacs: 1,
        num_small_keys: 2,
        treasures: [Treasure::BombsCounter].iter().cloned().collect(),
    }
    .into();
    outline.show();
}
