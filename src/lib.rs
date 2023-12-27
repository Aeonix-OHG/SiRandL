/*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: lib.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 25 12 2023
 * Modified: 27 12 2023
 * Modified By: Jan Simon Schmitt
 */
use core::arch::asm;

pub struct RandNum {
     seed: i64,
     a: i64,
     c: i64,
     m: i64
 }
 
 impl RandNum {
    pub fn new() -> Self {
        let mut lo: u64;
        let mut hi: u64;
        unsafe {
            asm!(
                "rdtsc",
                out("eax") lo,
                out("edx") hi,
            );
        }
        let seed = ((hi as u128) << 64) | lo as u128;
        let seed = seed as i64;
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
        let mut seed = RandNum::new();
        loop {
           println!("{}", seed.get(1, 1000));
        }
    }
}
