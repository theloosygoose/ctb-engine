pub mod game_handle;
pub mod math;
pub mod person;
pub mod teams;
pub mod weights;
pub mod traits;

use crate::teams::*;
use crate::game_handle::calculate::possession_loop;

use crate::game_handle::possession::Turnover;
use crate::person::PersonId;

use std::collections::HashMap;

fn main() {
    let team_0 = get_team_0();
    let team_1 = get_team_1();

    let qtr_time = 120;

    /* let mut n = 0;
    let mut counter:HashMap<String, u16> = HashMap::new();
    let mut num_passes:HashMap<usize, u16> = HashMap::new();

    while n < 120 {
        let data = possession_loop(&team_0, &team_1, qtr_time);

        match data.turnover {
            Turnover::ShotClock(_) => {
                let count = counter.entry("SHOTCLOC".to_string()).or_insert(0);
                *count += 1;
            },
            _ => {
                let count = counter.entry(data.shot_taker.unwrap().0).or_insert(0);
                *count +=1 ;
            }
        }
        let passes = num_passes.entry(data.passes.len()).or_insert(0);
        *passes +=1;

        n += 1; 
    }

    println!("{:#?}", counter);
    println!("{:#?}", num_passes); */
    let data = possession_loop(&team_0, &team_1, qtr_time);
    println!("{:#?}", data);

}
