// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use hudi_core::file_group::base_file::BaseFile;
use hudi_core::file_group::file_slice::FileSlice;
use hudi_core::util::collection::{split_into_n_chunks_clone, split_into_n_chunks_move};
use std::collections::BTreeSet;
use std::str::FromStr;

fn generate_file_slice(i: usize, logs_per_slice: usize) -> FileSlice {
    let id = format!(
        "{i:08x}-{i:08x}-{i:08x}-{i:08x}-{i:08x}-0",
        i = i
    );
    let base_name = format!("{id}_0-0_20250121000656060.parquet");
    let base = BaseFile::from_str(&base_name).unwrap();
    let mut logs = BTreeSet::new();
    for l in 0..logs_per_slice {
        let id = format!(
            "{i:08x}-{i:08x}-{i:08x}-{i:08x}-{i:08x}-0",
            i = i
        );
        let log = format!(".{id}_20250121000656060.log.{l}_0-1-1");
        logs.insert(hudi_core::file_group::log_file::LogFile::from_str(&log).unwrap());
    }
    FileSlice {
        base_file: base,
        log_files: logs,
        partition_path: "p=1".to_string(),
    }
}

fn make_slices(count: usize, logs_per_slice: usize) -> Vec<FileSlice> {
    (0..count).map(|i| generate_file_slice(i, logs_per_slice)).collect()
}

fn bench_split_slices(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_slices");

    // Parameter grid: (#slices, logs_per_slice, num_splits)
    let sizes = [64_usize, 256, 1024, 4096];
    let logs = [0_usize, 2, 8];
    let splits = [1_usize, 2, 4, 8, 32];

    for &n_slices in &sizes {
        for &n_logs in &logs {
            for &n_splits in &splits {
                let id = format!("N={} logs={} splits={}", n_slices, n_logs, n_splits);
                group.throughput(Throughput::Elements(n_slices as u64));

                // Clone-based benchmark
                group.bench_with_input(BenchmarkId::new("clone", &id), &id, |b, _| {
                    b.iter_batched(
                        || make_slices(n_slices, n_logs),
                        |v| {
                            let _ = split_into_n_chunks_clone(&v, n_splits);
                        },
                        BatchSize::SmallInput,
                    )
                });

                // Move-based benchmark
                group.bench_with_input(BenchmarkId::new("move", &id), &id, |b, _| {
                    b.iter_batched(
                        || make_slices(n_slices, n_logs),
                        |v| {
                            let _ = split_into_n_chunks_move(v, n_splits);
                        },
                        BatchSize::SmallInput,
                    )
                });
            }
        }
    }

    group.finish();
}

criterion_group!(benches, bench_split_slices);
criterion_main!(benches);


