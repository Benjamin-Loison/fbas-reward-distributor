# FBAS Reward Distributor

Framework for the computation of a node's influence and reward distribution in
a Federated Byzantine Agreement Systems (FBASs) like
[Stellar](https://www.stellar.org/).

## 1. Required tools

    - - A working [Rust](https://www.rust-lang.org) environment
        - Install: https://www.rust-lang.org/tools/install

## 2. Build and run
Compilitation and execution can be achieved in a single step as shown below

```
cargo run --release -- -a alg -r reward fbas-path [-i]
    - fbas-path: Path to file describing the FBAS
    - -i: Ignore inactive nodes in the FBAS.
    - -a alg: algorithm to use to determine node rankings.
    - -r reward`: reward value that is to be distributed
```

For example:

```
cargo run --release -- -a noderank -r 10 -i test_data/mobilecoin_nodes_2021-10-22.json
```

will compute how 10 units should be distributed among the nodes in the `mobilecoin_nodes_2021-10-22.json` using a graph-theoretic (Noderank) metric.

The same can be accomplished using game-theoretic measure, i.e. the 'Shapley - Shubik Power Index':

```
cargo run --release -- -a powerindex -r 10 -i test_data/mobilecoin_nodes_2021-10-22.json
```

The output is currently a Hashmap of <NodeID, (Ranking, Reward)>.
