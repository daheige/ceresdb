// Copyright 2023 The CeresDB Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lazy_static::lazy_static;
use prometheus::{exponential_buckets, register_histogram_vec, HistogramVec};

lazy_static! {
    // Buckets: 0, 0.01, .., 0.01 * 2^12
    pub static ref PARTITION_TABLE_WRITE_DURATION_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "partition_table_write_duration",
        "Histogram for write duration of the partition table in seconds",
        &["type"],
        exponential_buckets(0.01, 2.0, 13).unwrap()
        )
    .unwrap();

    // Buckets: 0, 0.01, .., 0.01 * 2^12
    pub static ref PARTITION_TABLE_PARTITIONED_READ_DURATION_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "partition_table_partitioned_read_duration",
        "Histogram for partitioned read duration of the partition table in seconds",
        &["type"],
        exponential_buckets(0.01, 2.0, 13).unwrap()
        )
    .unwrap();
}
