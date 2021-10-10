# Overview

# Dependencies

* Cargo
  Install it using [rustup]:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

* [Graphviz]
  ```sh
  apt-get install graphviz
  ```

* Image viewer
  It's nice to have one that can read a PNG image from stdin. I use `feh`.
  ```sh
  apt-get install feh
  ```

# Building

```sh
cargo build
```

# Running

To see all the intermediate stages of the generation of a given seed, run the following commands:

```sh
target/debug/tunics-algo --seed 12345 build-plan | dot -Tpng | feh -
target/debug/tunics-algo --seed 12345 build-sequence | dot -Tpng | feh -
target/debug/tunics-algo --seed 12345 feature-plan1 | dot -Tpng | feh -
target/debug/tunics-algo --seed 12345 feature-plan2 | dot -Tpng | feh -
target/debug/tunics-algo --seed 12345 room-plan | dot -Tpng | feh -
target/debug/tunics-algo --seed 12345 floor-plan | dot -Tpng | feh -
```

Here's an example of a larger dungeon:

```sh
target/debug/tunics-algo --items=bomb-bag --items=bow --items=flippers --small-keys=3 --traps=1 --fairies=1 --cul-de-sacs=1 floor-plan | dot -Tpng | feh -
```

[rustup]: https://rustup.rs/
[Graphviz]: https://graphviz.org/
