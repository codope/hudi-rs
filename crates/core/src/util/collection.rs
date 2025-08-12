/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use std::collections::HashMap;
use std::hash::Hash;

pub fn extend_if_absent<K, V>(target: &mut HashMap<K, V>, source: &HashMap<K, V>)
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    for (key, value) in source {
        target.entry(key.clone()).or_insert_with(|| value.clone());
    }
}

/// Split a slice into at most `n` chunks by cloning elements.
///
/// - Returns an empty vector if `items` is empty.
/// - If `n == 0`, it behaves as `n == 1`.
pub fn split_into_n_chunks_clone<T: Clone>(items: &[T], n: usize) -> Vec<Vec<T>> {
    if items.is_empty() {
        return Vec::new();
    }

    let n = std::cmp::max(1, n);
    let chunk_size = items.len().div_ceil(n);
    items
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Split a vector into at most `n` chunks by moving elements.
///
/// - Returns an empty vector if `items` is empty.
/// - If `n == 0`, it behaves as `n == 1`.
/// - Preserves input order.
pub fn split_into_n_chunks_move<T>(items: Vec<T>, n: usize) -> Vec<Vec<T>> {
    if items.is_empty() {
        return Vec::new();
    }

    let n = std::cmp::max(1, n);
    let total = items.len();
    let chunk_size = total.div_ceil(n);

    // Fast path: no split requested
    if chunk_size >= total {
        return vec![items];
    }

    let mut result: Vec<Vec<T>> = Vec::with_capacity(n);
    let mut current: Vec<T> = Vec::with_capacity(chunk_size);
    for item in items.into_iter() {
        current.push(item);
        if current.len() == chunk_size {
            result.push(current);
            current = Vec::with_capacity(chunk_size);
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}
