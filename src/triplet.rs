// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt;

#[derive(Eq, PartialOrd, Ord, PartialEq)]
pub struct Triplet {
    a: u64,
    b: u64,
    c: u64,
}

impl Triplet {
    pub fn new(mut members: Vec<u64>) -> Self {
        members.sort();
        Self {
            a: members[0],
            b: members[1],
            c: members[2],
        }
    }
    pub fn product(&self) -> u128 {
        self.a as u128 * self.b as u128 * self.c as u128
    }
    pub fn sum(&self) -> u128 {
        self.a as u128 + self.b as u128 + self.c as u128
    }
}

impl fmt::Display for Triplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}) = {} -> product {}",
            self.a,
            self.b,
            self.c,
            self.sum(),
            self.product()
        )
    }
}
