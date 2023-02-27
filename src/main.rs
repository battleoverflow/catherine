/*
    Project: Catherine Framework (https://github.com/azazellabs/catherine)
    Author: Azazel Labs (https://github.com/azazellabs)
    License: BSD 2-Clause
*/

use meta::banners;
use rand::Rng;

mod core;
mod modules;
mod catherine;
mod meta;

fn main() {
    // Randomizes vector call based on number of banners
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen_range(0, banners().len()).try_into().unwrap();

    // Returns the banner vector
    // Randomizes u8 integer (+ converts to usize) for random banner from vector
    catherine::init(&banners()[num as usize]);
    catherine::shutdown("You know what they say cowboy, easy come easy go...");
}