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
     m: i64
 }
 
 impl RandNum {
    pub fn new(seed: i64) -> Self {
        RandNum {
            seed,
            a: 1664525,
            c: 1013904223,
            m: 2i64.pow(32),
        }
    }
    
     
 
    pub fn get(&mut self, min: i64, max: i64) -> i64 {
         self.seed = (self.a * self.seed + self.c) % self.m;
         min + (self.seed % (max - min + 1))
     }
 }
 
// Tests all functions
#[cfg(test)]
mod tests {
    use crate::RandNum;

    #[test]
    fn it_works() {
        let mut seed = RandNum::new(82);
        loop {
            println!("{}", seed.get(1, 1000));
        }
    }
}
