/*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: lib.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 25 12 2023
 * Modified: 26 12 2023
 * Modified By: Jan Simon Schmitt
 */

pub struct RandNum {
     seed: i64,
     a: i64,
     c: i64,
     m: i64,
     min_value: i64,
     max_value: i64,
 }
 
 impl RandNum {
    pub fn new(min_value: i64, max_value: i64, seed: i64) -> Self {
        RandNum {
            seed,
            a: 1664525,
            c: 1013904223,
            m: 2i64.pow(32),
            min_value,
            max_value,
        }
    }
    
     
 
    pub fn get(&mut self) -> i64 {
         self.seed = (self.a * self.seed + self.c) % self.m;
         self.min_value + (self.seed % (self.max_value - self.min_value + 1))
     }
 }
 
// Tests all functions
#[cfg(test)]
mod tests {
    use crate::RandNum;

    #[test]
    fn it_works() {
        let mut seed = RandNum::new(1, 100000000000000000, 1);
        loop {
            println!("{}", seed.get());
        }
    }
}
