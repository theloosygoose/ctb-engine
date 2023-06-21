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

    possession_loop(&team_0, &team_1, qtr_time);
    //let data = possession_loop(&team_0, &team_1, qtr_time);
    //println!("{:#?}", data);

}
