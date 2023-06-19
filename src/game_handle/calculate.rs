use super::constants::*;
use super::possession::*;
use crate::person::{PersonId, Person};

use crate::person::matchup::*;
use crate::person::previews::*;

use crate::traits::random_person::WeightedRandom;
use crate::traits::search::Searchable;

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
        let initiator = &o_initiator_previews.random_person();
        let i_matchup = matchups.get_player(initiator).1;

        let initiator_creation = o_driving_previews.get_player(initiator);
        let primary_defender_onball = d_on_ball_previews.get_player(&i_matchup);

        println!("{:#?} is the initiator", &initiator);
        println!("He is guarded by {:#?}", &i_matchup);
        let openess = find_openess( &initiator_creation, &primary_defender_onball, &o_off_ball_previews, &d_off_ball_previews);;

        
        println!("{:?}", openess);

        duration += 20; 
    }

    possession
}


//So Based off ball handler ability to get open
fn find_openess(
                o_creation: &OffPreview,
                d_onball: &DefPreview,
                offense: &Vec<OffPreview>, 
                defense: &Vec<DefPreview>,
                ) -> [f32; 5] {
    let mut openess: [f32; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
    let mut average_def = 0.0;

    defense.iter().for_each(|preview| average_def += preview.1);

    average_def = average_def / defense.len() as f32;

    let mut i:usize = 1;

    openess[0] = o_creation.1 - d_onball.1;

    offense.iter().for_each(|preview|{
        if preview.0 != o_creation.0 {
            openess[i + 0] = (preview.1 + (openess[0] / 50.0)) - average_def;
            i += 1;
        };
    });

    openess
}
