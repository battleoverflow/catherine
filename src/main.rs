/*
    Owner: Catherine Framework (https://github.com/CatherineFramework)
    Project: Catherine
    License: BSD 2-Clause
*/

use catherine::VERSION;
use rand::Rng;

mod core;
mod modules;
mod catherine;

fn banner_data() -> Vec<String> {

    // Catherine banners
    let banners: Vec<String> = vec![
        format!(r"
         _____       _   _               _            
        /  __ \     | | | |             (_)           
        | /  \/ __ _| |_| |__   ___ _ __ _ _ __   ___ 
        | |    / _` | __| '_ \ / _ \ '__| | '_ \ / _ \
        | \__/\ (_| | |_| | | |  __/ |  | | | | |  __/
         \____/\__,_|\__|_| |_|\___|_|  |_|_| |_|\___|

                                              v{}
        ", VERSION),

        format!(r"
          |\                     /)
        /\_\\__               (_//
       |   `>\-`     _._       //`)
        \ /` \\  _.-`:::`-._  //
         `    \|`    :::    `|/
               |     :::     |
               |.....:::.....|
               |:::::::::::::|
               |     :::     |
               \     :::     /
                \    :::    /
                 `-. ::: .-'
                  //`:::`\\
                 //   '   \\
                |/         \\
                      
              Catherine | v{}
        ", VERSION),
    ];

    return banners;
}

fn main() {
    // Randomizes vector call based on number of banners
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen_range(0, banner_data().len()).try_into().unwrap();

    // Returns the banner vector
    // Randomizes u8 integer (+ converts to usize) for random banner from vector
    catherine::init(&banner_data()[num as usize]);
    catherine::shutdown("Shutting down...");
}