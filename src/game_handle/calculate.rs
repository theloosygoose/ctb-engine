use super::constants::*;
use super::possession::*;
use crate::person::{PersonId, Person};

use crate::person::matchup::*;
use crate::person::previews::*;

use crate::traits::random_person::WeightedRandom;
use crate::traits::search::Searchable;

use std::ops::Neg;

pub enum Action {
    Three(u16),
    Midrange(u16),
    Close(u16),
    Layup(u16),
    Create(u16),
    Pass(PersonId, u16),
}

pub fn possession_loop(
    offense: &Vec<Person>,
    defense: &Vec<Person>,
    qtr_time: u16,
) -> PossesionData {

    let shot_clock = if qtr_time < SHOT_CLOCK { qtr_time } else { SHOT_CLOCK };

    //INITILIZE Posession 
    let mut possession = PossesionData::new();

    //INITILIZE PREVIEWS

    let o_off_ball_previews = OffPreview::off_previews(offense, OffVal::OffBall);
    let d_off_ball_previews = DefPreview::def_previews(defense, DefVal::OffBall);

    let o_initiator_previews = OffPreview::off_previews(offense, OffVal::Initiator);
    let d_on_ball_previews = DefPreview::def_previews(defense, DefVal::OnBall);

    let o_spacing_previews = OffPreview::off_previews(offense, OffVal::FloorSpacing);
    let o_driving_previews = OffPreview::off_previews(offense, OffVal::Driving);
    let o_passing_previews = OffPreview::off_previews(offense, OffVal::Passing);

    //INITILIZE MATCHUPS
    let matchups = Matchup::gen(&o_driving_previews, &d_on_ball_previews);

    let mut initiator = o_initiator_previews.random_person();
    let mut i_matchup:PersonId;

    loop {
        i_matchup = matchups.get_player(&initiator).1;
        let mut time_left = shot_clock - &possession.duration;
        if time_left == shot_clock {
            time_left -= 5;
        };

        println!("TIME LEFT:: {}", shot_clock - &possession.duration);
        //println!("\n");

        let initiator_creation = o_driving_previews.get_player(&initiator);
        let initiator_passing = o_passing_previews.get_player(&initiator);
        let primary_defender_onball = d_on_ball_previews.get_player(&i_matchup);

        let on_ball_openess = &initiator_creation.1 - &primary_defender_onball.1;

        //println!("{:#?} is the initiator", &initiator);
        //println!("He is guarded by {:#?}", &i_matchup);
        //println!("ON BALL CREATION {}", &on_ball_openess);


        let openess = team_openess(
            on_ball_openess, 
            initiator_passing.1,
            &initiator_creation.0, 
            &o_off_ball_previews, 
            &o_spacing_previews, 
            &d_off_ball_previews,
            time_left
        );

        match action(&openess, &offense.get_player(&initiator), &defense.get_player(&i_matchup)) {
            Action::Pass(reciever, duration) => {
                possession.pass(initiator, reciever.clone(), duration);
                initiator = reciever;
            },
            _ => {
                //println!("{:#?} Does thier own thing", initiator.0);
                possession.shot(ShotType::Three(true), initiator, i_matchup,5);
                break;
            }
        };

        println!("\n");
        println!("{:?}", openess);

        if possession.duration >= shot_clock {
            possession.turnover(Turnover::ShotClock(initiator), 0);
            break;
        }
    }

    //println!("{:?}", &possession.passes);
    possession

}

//So Based off ball handler ability to get open
fn team_openess(
                on_ball_openess: f32,
                on_ball_passing_ability: f32,
                on_ball_player: &PersonId,
                offense_off_ball: &Vec<OffPreview>, 
                offense_spacing: &Vec<OffPreview>, 
                defense_offball: &Vec<DefPreview>,
                time_left: u16,
                ) -> Vec<(PersonId, f32)> {

    let mut openess: Vec<(PersonId, f32)> = Vec::with_capacity(5);
    let mut average_def = 0.0;
    let mut average_spacing = 0.0;

    defense_offball.iter().for_each(|preview| average_def += preview.1);
    average_def = average_def / defense_offball.len() as f32;
    //println!("DEFENSE OFF BALL: {}", average_def);
    //println!("\n");

    offense_spacing.iter().for_each(|preview| average_spacing += preview.1);
    average_spacing = average_spacing / offense_spacing.len() as f32;

    //println!("OFFENSE SPACING: {}", average_spacing);
    //println!("\n");

    //println!("PASSING ON BALL: {}", on_ball_passing_ability);
    //println!("\n");

    offense_off_ball.iter().for_each(|preview| {
        if &preview.0 != on_ball_player {
            let open_value = (preview.1 + on_ball_passing_ability) - average_def;
            openess.push((preview.0.clone(), open_value));
        } else {
            openess.push((preview.0.clone(), (1.23_f32.powi( (time_left as i32 - 30).neg() ) + 50.0) + (on_ball_openess - ((100.0 - average_spacing)/90.0))));
        }
    });

    openess
}

fn action(team_openess: &Vec<(PersonId, f32)>, offensive_player: &Person, defender: &Person) -> Action {
    let player = team_openess.random_person();

    if player == offensive_player.person_id {
        //println!("{:#?} DOES HIS OWN ACTION", player);
        Action::Close(3)

    } else {
        //println!("{:#?} PASSES THE BALL TO {:#?}", offensive_player.person_id.0, player.0);
        Action::Pass(player, 4)
    }
}

