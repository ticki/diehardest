//! Analysis of pseudorandom streams.

use std::cmp;
use std::collections::HashSet;

use Random;

/// A analysis report extracted from some stream.
pub struct Report {
    /// The index in which the first number is returned again.
    ///
    /// If it is never found again, the value is 0.
    cycle_length: u32,
    /// The number of colliding numbers found in the sample of the stream.
    collisions: u8,
    /// The bit dependency matrix.
    ///
    /// This contains the probability that bit `x` is set if bit `y` is, i.e. `p(y|x)`.
    dependency_matrix: [[u32; 64]; 64],
    /// The distribution of the sample, modulo 4096.
    distribution: [u16; 4096],
}

impl Default for Report {
    fn default() -> Report {
        Report {
            cycle_length: 0,
            collisions: 0,
            dependency_matrix: [[0; 64]; 64],
            distribution: [0; 4096],
        }
    }
}

impl Report {
    /// Investigate a random stream and create a report.
    pub fn new<R: Random>(mut rand: R) -> Report {
        let mut report = Report::default();
        let mut set = HashSet::new();

        let start = rand.get_random();
        for n in 0..1 << 16 {
            // Collect a random number.
            let r = rand.get_random();

            // Update the bit depedency matrix.
            for x in 0..64 {
                for y in 0..64 {
                    report.dependency_matrix[x][y] += ((r & (1 << x) == 0) <= (r & (1 << y) == 0)) as u32;
                }
            }

            // Increment the distribution entry.
            report.distribution[r as usize % 4096] += 1;

            // If it returned to the first number, set the cycle length.
            if report.cycle_length == 0 && r == start {
                report.cycle_length = n;
            }

            // Insert the random number into the set and update the collision number.
            report.collisions += (!set.insert(r)) as u8;
        }

        report
    }

    /// Get the final score of this report.
    pub fn get_score(&self) -> Score {
        Score {
            // The cycle should not be less than the sample size.
            cycle: if self.cycle_length == 0 {
                255
            } else {
                0
            },
            // Ideally, there should be no collisions in our sample. Applying the birthday problem
            // still gives us very small probability of such a collision occuring.
            collision: match self.collisions {
                0 => 255,
                1 => 20,
                _ => 0,
            },
            bit_dependency: {
                // Calculate the minimum and maximum entry of the dependency matrix.
                let mut max = 0;
                let mut min = !0;
                for x in 0..64 {
                    for y in 0..64 {
                        max = cmp::max(self.dependency_matrix[x][y], max);
                        min = cmp::min(self.dependency_matrix[x][y], min);
                    }
                }

                // Rate it based on it's distance to the ideal value.
                let pmin = match 65536 - min as i32 {
                    0...4 => 127,
                    4...6 => 126,
                    6...16 => 120,
                    16...32 => 90,
                    32...64 => 50,
                    64...80 => 20,
                    _ => 0,
                };

                // Rate it based on it's distance to the ideal value.
                let pmax = match min as i32 - 65536 {
                    0...4 => 128,
                    4...6 => 126,
                    6...16 => 120,
                    16...32 => 90,
                    32...64 => 50,
                    64...80 => 20,
                    _ => 0,
                };

                pmin + pmax
            },
            distribution: {
                // Calculate the minimum and maximum entry of the distribution array.
                let mut max = 0;
                let mut min = !0;
                for i in 0..4096 {
                    max = cmp::max(self.distribution[i], max);
                    min = cmp::min(self.distribution[i], min);
                }

                // Rate it based on it's distance to the ideal value.
                let pmin = match 32 - min as i32 {
                    0...4 => 127,
                    4...6 => 126,
                    6...10 => 110,
                    10...15 => 70,
                    15...18 => 50,
                    18...20 => 30,
                    20...32 => 20,
                    _ => 0,
                };

                // Rate it based on it's distance to the ideal value.
                let pmax = match min as i32 - 32 {
                    0...4 => 128,
                    4...6 => 126,
                    6...10 => 110,
                    10...15 => 70,
                    15...18 => 50,
                    18...20 => 30,
                    20...32 => 20,
                    _ => 0,
                };

                pmin + pmax
            },
        }
    }
}

/// The score of some report.
pub struct Score {
    /// The quality of the cycle length.
    cycle: u8,
    /// The quality of occurence of collisions.
    collision: u8,
    /// The quality of the BIC.
    bit_dependency: u8,
    /// The quality of the distribution.
    distribution: u8,
}

impl Score {
    /// Sum the scores together to a single integer.
    pub fn total(self) -> u16 {
        self.cycle as u16
            + self.collision as u16
            + self.bit_dependency as u16
            + self.distribution as u16
    }
}
