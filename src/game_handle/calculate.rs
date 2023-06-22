use super::constants::*;
use super::possession::*;
use crate::person::{PersonId, Person};

use crate::person::matchup::*;
use crate::person::previews::*;

use crate::traits::random_person::WeightedRandom;
use crate::traits::search::Searchable;

use rand::Rng;


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
    let o_creation_previews = OffPreview::off_previews(offense, OffVal::Creation);
    let o_passing_previews = OffPreview::off_previews(offense, OffVal::Passing);

    //INITILIZE MATCHUPS
    let matchups = Matchup::gen(&o_creation_previews, &d_on_ball_previews);

    let mut initiator = o_initiator_previews.random_person();
    let mut i_matchup:PersonId;

    loop {
        //GET THE MATCHUP FOR THE CURRENT BALL HANDLER
        i_matchup = matchups.get_player(&initiator).1;
        let time_left = shot_clock - &possession.duration;

        println!("TIME LEFT:: {}", shot_clock - &possession.duration);
        println!("\n");

        let initiator_creation = o_creation_previews.get_player(&initiator);
        let initiator_passing = o_passing_previews.get_player(&initiator);
        let primary_defender_onball = d_on_ball_previews.get_player(&i_matchup);

        let openess = team_openess(
            &initiator_creation,
            &initiator_passing,

            primary_defender_onball.1,

            &o_off_ball_previews, 

            &d_off_ball_previews,
        );

        match action(&openess, &o_creation_previews, offense.get_player(&initiator), defense.get_player(&primary_defender_onball.0), time_left) {
            Action::Pass(passesto) => {
                possession.pass(initiator.clone(), passesto.clone(), 4);
                println!("{:?} PASSESS THE BALL TO {:?}", &initiator, &passesto);
                initiator = passesto.clone();
            },
            Action::CreateDrive(prob)=> {
                possession.shot(ShotType::Layup(true), initiator.clone(), primary_defender_onball.0.clone(), prob.round() as u16);
                break;
            },
            Action::CreateShot(prob)=> {
                possession.shot(ShotType::Three(true), initiator.clone(), primary_defender_onball.0.clone(), prob.round() as u16);
                break;
            },
            _ => {
                todo!();
            }
        };

        if possession.duration >= shot_clock {
            possession.turnover(Turnover::ShotClock(initiator), 0);
            break;
        }
    }

    possession

}
#[derive(Debug)]
pub struct Openess(pub PersonId, pub f32);

#[derive(Debug)]
struct PassTo(PersonId, f32);

//So Based off ball handler ability to get open
fn team_openess(
                initiatior:&OffPreview,
                on_ball_passing:&OffPreview,

                on_ball_defender:f32,

                offense_off_ball: &Vec<OffPreview>, 

                defense_offball: &Vec<DefPreview>,

                ) -> Vec<Openess> {

    let mut openess:Vec<Openess> = Vec::with_capacity(4);

    //GET AVERAGE DEFENSE OFF BALL
    let mut def_avg = 0.0;
    defense_offball.iter().for_each(|defender|{
        def_avg += defender.1;
    });

    let mut rand_thread = rand::thread_rng();
    let initiator_mod = rand_thread.gen_range(0.0..2.0);
    let defender_mod = rand_thread.gen_range(0.0..2.0);

    //Calculate Creation win or loss
    let on_ball_creation_mod = (initiatior.1 * initiator_mod) - (on_ball_defender * defender_mod);

    let def_mod = rand::thread_rng().gen_range(-20..20);
    def_avg = (def_avg / defense_offball.len() as f32) + def_mod as f32;

    //Who is open?
    offense_off_ball.iter().for_each(|off_ball|{
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0.0..1.0);

        if off_ball.0 != initiatior.0 {
            let val = ((off_ball.1 - def_avg) * num) + (on_ball_creation_mod/60.0) + (on_ball_passing.1/50.0);
            openess.push(Openess(off_ball.0.clone(), val));
        };

    });

    openess
}

pub enum Action {
    Three(f32),
    Midrange(f32),
    Close(f32),
    Layup(f32),
    CreateDrive(f32),
    CreateShot(f32),
    Pass(PersonId),
}


fn action(team_openess: &Vec<Openess>, team_creation_abillity: &Vec<OffPreview> ,ball_handler: Person, defender: Person, time_left: u16) -> Action {
    println!("{:?}", team_openess);
    let possible_playerid = team_openess.random_person();
    let possible_openess = team_openess.get_player(&possible_playerid);

    let handler_creation = team_creation_abillity.get_player(&ball_handler.person_id);
    let possible_pass_creation = team_creation_abillity.get_player(&possible_playerid);

    let handler_intangibles = ball_handler.intangibles;
    let handler_personality = ball_handler.personality;

    let defender_intangibles = defender.intangibles;

    //SMART DECISION CHECK
    let rand_intelligence = rand::thread_rng().gen_range(-40..50) as i32 + handler_personality.intelligence as i32;
    let smart_decision = if rand_intelligence > 50 { true } else { false };

    //DOG CHECK
    let rand_dog = rand::thread_rng().gen_range(-30..50) as i32 + handler_personality.dog as i32;
    let dog_check = if rand_dog > 30 { true } else { false };

    //Check to see if the player he might pass to is better at creating than him
    //IF HE IS THEN HAVE AN INTELLIGENCE CHECK TO SEE IF HE PASSES THE BALL
    if possible_pass_creation.1 > handler_creation.1 {
        //PLAYER DOES NOT HAVE BETTER CREATION THAN POSSIBLE PASS PLAYER
        if smart_decision {
            println!("{:?} Makes the smart decision to pass to {:?}", &ball_handler.person_id, possible_playerid);
            Action::Pass(possible_playerid.clone())
        } else {
            println!("{:?} Makes the DUMB decision to create SHOULD PASS TO {:?}", &ball_handler.person_id, possible_playerid);
            match shoot_or_drive(&ball_handler) {
                ShotDrive::CreateDrive => {
                    Action::CreateDrive(2.0)
                }, 
                ShotDrive::CreateShot => {
                    Action::CreateShot(2.0)
                }
            }
        }
    } else {
        //PLAYER DOES HAVE BETTER CREATION THAN POSSIBLE PASS PLAYER
        if dog_check {
            println!("DOG CHECK PASS !!! {:?}", &ball_handler.person_id);
            match shoot_or_drive(&ball_handler) {
                ShotDrive::CreateDrive => {
                    Action::CreateDrive(2.0)
                }, 
                ShotDrive::CreateShot => {
                    Action::CreateShot(2.0)
                }
            }
        } else {
            println!("DOG CHECK FAILED {:?} WAS TOO AFRAID OF THE MOMENT", &ball_handler.person_id);
            // DOG CHECK FAILED PLAYER PASSES THE BALL EVEN THOUGH HIS CREATION IS BETTER
            Action::Pass(possible_playerid.clone())
        }

    }
}

#[derive(PartialEq, Eq)]
enum ShotDrive {
    CreateShot,
    CreateDrive, 
}

fn shoot_or_drive(player: &Person) -> ShotDrive {

    let will_shoot = player.intangibles.touch + player.intangibles.shot_form + player.intangibles.burst;

    let will_drive = player.intangibles.touch + player.intangibles.burst + player.intangibles.strength;

    let rand_intelligence = rand::thread_rng().gen_range(-40..50) as i32 + player.personality.intelligence as i32;

    let smart_decision = if rand_intelligence > 50 {
        true
    } else {
        false
    };
    if will_shoot > will_drive {
        if smart_decision {
            ShotDrive::CreateShot

        } else {
            ShotDrive::CreateDrive

        }
    } else {
        if smart_decision {
            ShotDrive::CreateDrive
        } else {
            ShotDrive::CreateShot
        }
    } 
}


