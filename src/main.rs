pub mod game_handle;
pub mod math;
pub mod person;
pub mod team;
pub mod teams;
pub mod weights;

use crate::teams::*;
use crate::team::*;
use crate::person::previews::*;
use crate::person::matchup::Matchup;

fn main() {
    let team_0 = get_team_0();
    let team_1 = get_team_1();

    let team_0_previes = team_0.off_previews(OffVal::Driving);
    let team_1_previes = team_1.def_previews(DefVal::OnBall);

    let matchups = Matchup::gen(&team_0_previes, &team_1_previes);

    println!("----MATCHUPS----");
    println!("{:#?}", matchups);
}
