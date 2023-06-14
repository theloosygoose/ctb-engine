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

    let team_0_off_previes = team_0.clone().off_previews(OffVal::Driving);
    let team_1_def_previes = team_1.clone().def_previews(DefVal::OnBall);

    let team_0_def_previes = team_0.def_previews(DefVal::OnBall);
    let team_1_off_previes = team_1.off_previews(OffVal::Driving);

    let team_1_matchups = Matchup::gen(&team_0_off_previes, &team_1_def_previes);
    let team_0_matchups = Matchup::gen(&team_1_off_previes, &team_0_def_previes);

    println!("----MATCHUPS----");
    println!("{:#?}", team_1_matchups);
    println!("{:#?}", team_0_matchups);
}
