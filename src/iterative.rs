// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::result::Result;
use crate::triplet::Triplet;

pub fn find(sum: u64) -> Result {
    let mut result = Result::new("Iterative");

    for a in 1..(sum as f64 / 3.0).ceil() as u64 {
        for b in a..((sum as f64 - a as f64) / 2.0).ceil() as u64 {
            result.add_iteration();

            let c = sum - a - b;
            if a * a + b * b == c * c {
                result.add_triplet(Triplet::new(vec![a, b, c]));
            }
        }
    }

    result.finalize();
    result
}
