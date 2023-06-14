pub mod game_handle;
pub mod math;
pub mod person;
pub mod team;
pub mod teams;
pub mod weights;

use crate::teams::*;
use crate::person::previews::*;
use crate::person::matchup::Matchup;

fn main() {
    let team_0 = get_team_0();
    let team_1 = get_team_1();

    let team_0_off_previes = OffPreview::off_previews(&team_0, OffVal::Driving);
    let team_1_def_previes = DefPreview::def_previews(&team_1, DefVal::OffBall);

    let team_0_def_previes = DefPreview::def_previews(&team_0, DefVal::OnBall);
    let team_1_off_previes = OffPreview::off_previews(&team_1, OffVal::Driving);

    let team_1_matchups = Matchup::gen(&team_0_off_previes, &team_1_def_previes);
    let team_0_matchups = Matchup::gen(&team_1_off_previes, &team_0_def_previes);

    println!("----MATCHUPS----");
    println!("----TEAM_1 DEFENSE----");
    println!("{:#?}", team_1_matchups);
    println!("----TEAM_0 DEFENSE----");
    println!("{:#?}", team_0_matchups);
}
