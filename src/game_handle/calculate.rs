use super::constants::*;
use super::possession::*;
use crate::person::{PersonId, Person};

use crate::person::matchup::*;
use crate::person::previews::*;

use crate::traits::random_person::WeightedRandom;
use crate::traits::search::Searchable;

use rand::Rng;

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
    let o_creation_previews = OffPreview::off_previews(offense, OffVal::Creation);
    let o_passing_previews = OffPreview::off_previews(offense, OffVal::Passing);

    //INITILIZE MATCHUPS
    let matchups = Matchup::gen(&o_creation_previews, &d_on_ball_previews);

    let mut initiator = o_initiator_previews.random_person();
    let mut i_matchup:PersonId;

    loop {
        //GET THE MATCHUP FOR THE CURRENT BALL HANDLER
        i_matchup = matchups.get_player(&initiator).1;
        let mut time_left = shot_clock - &possession.duration;

        if time_left == shot_clock {
            time_left -= 5;
        };

        //println!("TIME LEFT:: {}", shot_clock - &possession.duration);
        //println!("\n");

        let initiator_creation = o_creation_previews.get_player(&initiator);
        let initiator_passing = o_passing_previews.get_player(&initiator);
        let primary_defender_onball = d_on_ball_previews.get_player(&i_matchup);

        //println!("{:#?} is the initiator", &initiator);
        //println!("He is guarded by {:#?}", &i_matchup);
        //println!("ON BALL CREATION {}", &on_ball_openess);
        let passes = possession.passes.len();

        let openess = team_openess(
            &initiator_creation,

            primary_defender_onball.1,

            &o_off_ball_previews, 
            &o_spacing_previews, 

            &d_off_ball_previews,
        );

        //match action(&openess, &offense.get_player(&initiator), &defense.get_player(&i_matchup)) {

            //Action::Pass(reciever, duration) => {
             //   possession.pass(initiator, reciever.clone(), duration);
              //  initiator = reciever;
            //},
            //_ => {
                //println!("{:#?} Does thier own thing", initiator.0);
             //   possession.shot(ShotType::Three(true), initiator, i_matchup,5);
              //  break;
            //}
        //};

        println!("\n");
        println!("{:#?}", openess);
        println!("{:#?}", initiator.0);
        println!("{:#?}", primary_defender_onball.0.0);

        //if possession.duration >= shot_clock {
        //    possession.turnover(Turnover::ShotClock(initiator), 0);
        //    break;
        //}
        break;
    }

    //println!("{:?}", &possession.passes);
    possession

}
#[derive(Debug)]
struct Openess(PersonId, f32);

#[derive(Debug)]
struct PassTo(PersonId, f32);

//So Based off ball handler ability to get open
fn team_openess(
                initiatior:&OffPreview,

                on_ball_defender:f32,

                offense_off_ball: &Vec<OffPreview>, 
                offense_spacing: &Vec<OffPreview>, 

                defense_offball: &Vec<DefPreview>,

                ) -> Vec<Openess> {

    let mut openess:Vec<Openess> = Vec::with_capacity(5);

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
            let val = ((off_ball.1 - def_avg) * num) + (on_ball_creation_mod/50.0);

            openess.push(Openess(off_ball.0.clone(), val));
        } else {
            let val = on_ball_creation_mod;
            openess.push(Openess(off_ball.0.clone(), val));
        }
    });

    openess
}

//fn action(team_openess: &Vec<(PersonId, f32)>, offensive_player: &Person, defender: &Person) -> Action {
 //   let player = team_openess.random_person();

  //  if player == offensive_player.person_id {
        //println!("{:#?} DOES HIS OWN ACTION", player);
   //     Action::Close(3)

    //} else {
        //println!("{:#?} PASSES THE BALL TO {:#?}", offensive_player.person_id.0, player.0);
     //   Action::Pass(player, 4)
    //}
//}

