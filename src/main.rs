extern crate diehardest;

use std::io;
use std::io::Read;

/// A RNG that reads from the standard input.
struct StdinRng {
    stdin: io::Stdin,
}

impl Clone for StdinRng {
    fn clone(&self) -> StdinRng {
        StdinRng {
            stdin: io::stdin(),
        }
    }
}

impl diehardest::Random for StdinRng {
    fn get_random(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.stdin.read(&mut buf).unwrap();

        let mut x = 0;
        for &i in &buf {
            x <<= 8;
            x |= i as u64;
        }

        x
    }
}

fn main() {
    println!("score: {}", diehardest::crush(StdinRng {
        stdin: io::stdin(),
    }));
}
