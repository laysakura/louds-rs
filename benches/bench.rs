#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Duration;

fn c() -> Criterion {
    Criterion::default()
        .sample_size(10) // must be >= 10 for Criterion v0.3
        .warm_up_time(Duration::from_secs(1))
        .with_plots()
}

fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

mod louds {
    use criterion::{BatchSize, Criterion};
    use louds_rs::{Louds, LoudsIndex, LoudsNodeNum};

    const NS: [u64; 5] = [1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15];

    fn generate_binary_tree_lbs_bits(n_nodes: u64) -> Vec<bool> {
        assert!(
            NS.iter().any(|n| n - 1 == n_nodes),
            "Only 2^m - 1 nodes (complete binary tree) is supported"
        );

        let mut bits = vec![true, false];

        // Nodes
        for _ in 1..=(n_nodes / 2) {
            bits.append(&mut vec![true, true, false]);
        }

        // Leaves
        for _ in (n_nodes / 2 + 1)..=(n_nodes) {
            bits.push(false);
        }

        bits
    }

    fn generate_binary_tree_lbs_string(n_nodes: u64) -> String {
        generate_binary_tree_lbs_bits(n_nodes)
            .iter()
            .map(|bit| if *bit { '1' } else { '0' })
            .collect()
    }

    pub fn from_bits_benchmark(_: &mut Criterion) {
        let times = 10;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds::from::<&[bool]>(&[...(bin tree of N nodes)]) {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || generate_binary_tree_lbs_bits(n - 1),
                    |bits| {
                        for _ in 0..times {
                            let _ = Louds::from(&bits[..]);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn from_str_benchmark(_: &mut Criterion) {
        let times = 10;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds::from::<&str>(\"...(bin tree of N nodes)\") {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || generate_binary_tree_lbs_string(n - 1),
                    |s| {
                        for _ in 0..times {
                            let _ = Louds::from(s.as_str());
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn node_num_to_index_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::node_num_to_index() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bits = generate_binary_tree_lbs_bits(n - 1);
                        Louds::from(&bits[..])
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.node_num_to_index(LoudsNodeNum(n - 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn index_to_node_num_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::index_to_node_num() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bits = generate_binary_tree_lbs_bits(n - 1);
                        Louds::from(&bits[..])
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.index_to_node_num(LoudsIndex(n / 2 + 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn parent_to_children_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::parent_to_children() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bits = generate_binary_tree_lbs_bits(n - 1);
                        Louds::from(&bits[..])
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.parent_to_children(LoudsNodeNum(n - 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn child_to_parent_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::child_to_parent() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bits = generate_binary_tree_lbs_bits(n - 1);
                        Louds::from(&bits[..])
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.child_to_parent(LoudsIndex(n / 2 + 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }
}

criterion_group!(
    benches,
    louds::from_bits_benchmark,
    louds::from_str_benchmark,
    louds::node_num_to_index_benchmark,
    louds::index_to_node_num_benchmark,
    louds::parent_to_children_benchmark,
    louds::child_to_parent_benchmark,
);
criterion_main!(benches);
