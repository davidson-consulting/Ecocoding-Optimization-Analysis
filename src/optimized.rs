// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use itertools::Itertools;
use primefactor::PrimeFactors;

use crate::result::Result;
use crate::triplet::Triplet;

pub fn find(sum: u64) -> Result {
    let mut result = Result::new("Optimized");

    let p_factors = PrimeFactors::from(sum as u128 / 2).to_vec();

    for k_factors in p_factors
        .iter()
        .powerset()
        .unique()
        .collect::<Vec<_>>()
        .iter()
    {
        let k = k_factors.clone().into_iter().product::<u128>() as u64;

        let mut r_factors = p_factors.clone();
        for f in k_factors {
            if let Some(pos) = r_factors.iter().position(|x| *x == **f) {
                r_factors.remove(pos);
            }
        }

        for m_factors in r_factors
            .iter()
            .powerset()
            .unique()
            .collect::<Vec<_>>()
            .iter()
        {
            result.add_iteration();

            let m = m_factors.clone().into_iter().product::<u128>() as u64;
            let mut n = ((sum as f64 / 2.0) / k as f64 / m as f64) as u64;

            if n >= 2 * m || m >= n {
                continue;
            }
            n -= m;

            result.add_triplet(Triplet::new(vec![
                k * (m * m - n * n),
                k * 2 * m * n,
                k * (m * m + n * n),
            ]));
        }
    }

    result.finalize();
    result
}
