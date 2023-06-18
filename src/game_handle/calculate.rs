use super::constants::*;
use super::possession::*;
use crate::person::{PersonId, Person};

use crate::person::matchup::*;
use crate::person::previews::*;

pub enum Action {
    Shoot,
    Create,
    Pass,
}

pub fn possession_loop(
    offense: &Vec<Person>,
    defense: &Vec<Person>,
    qtr_time: u16,
) -> PossesionData {
    let shot_clock = if qtr_time < SHOT_CLOCK { qtr_time } else { SHOT_CLOCK };
    let mut duration = 0;

    let mut passes: Vec<PersonId> = vec![];

    let mut possession = PossesionData {
        ongoing: true,
        shot_type: ShotType::None,
        shot_taker: None,
        defender: None,
        foul: Foul::None,
        points_scored: 0, 
        rebounder: None, 
        turnover: Turnover::None,

        duration,
        passes, 
    };

    let o_off_ball_previews = OffPreview::off_previews(offense, OffVal::OffBall);
    let d_off_ball_previews = DefPreview::def_previews(defense, DefVal::OffBall);

    let o_initiator_previews = OffPreview::off_previews(offense, OffVal::Initiator);
    let d_on_ball_previews = DefPreview::def_previews(defense, DefVal::OnBall);

    let o_spacing_previews = OffPreview::off_previews(offense, OffVal::FloorSpacing);
    let o_driving_previews = OffPreview::off_previews(offense, OffVal::Driving);

    let matchups = Matchup::gen(&o_driving_previews, &d_on_ball_previews);

    while duration < shot_clock {
        let initiator = &o_initiator_previews.pick_random_val();
        println!("{:#?} is the initiator", initiator);



    }

    possession
}


//So Based off ball handler ability to get open
fn find_openess(matchup: Matchup, offense: &Vec<OffPreview>, defense: &Vec<DefPreview>) -> [f32; 5] {
    let mut openess: [f32; 5] = todo!();


    openess
}

