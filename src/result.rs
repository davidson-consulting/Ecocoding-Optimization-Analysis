// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt;

use crate::triplet::Triplet;

pub struct Result {
    name: String,
    iterations: u128,
    triplets: Vec<Triplet>,
}

impl Result {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            iterations: 0,
            triplets: Vec::new(),
        }
    }
    pub fn add_iteration(&mut self) {
        self.iterations += 1;
    }
    pub fn add_triplet(&mut self, triplet: Triplet) {
        if !self.triplets.contains(&triplet) {
            self.triplets.push(triplet);
        }
    }
    pub fn nb_triplets(&self) -> usize {
        self.triplets.len()
    }
    pub fn finalize(&mut self) {
        self.triplets.sort();
    }
    fn borrow_triplets(&self) -> Triplets {
        //self.triplets.sort();
        Triplets(&self.triplets)
    }
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.triplets.len() {
            0 => write!(
                f,
                "{}: No solution found (in {} iterations)",
                self.name, self.iterations
            ),
            _ => write!(
                f,
                "{}: {} solutions found (in {} iterations)\n{}",
                self.name,
                self.triplets.len(),
                self.iterations,
                self.borrow_triplets()
            ),
        }
    }
}

struct Triplets<'a>(pub &'a Vec<Triplet>);

impl<'a> fmt::Display for Triplets<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, triplet| {
            result.and_then(|_| writeln!(f, "  {}", triplet))
        })
    }
}
