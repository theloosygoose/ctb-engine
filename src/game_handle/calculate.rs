use super::constants::*;
use super::possession::*;
use crate::person::Person;
use crate::person::matchup::*;
use crate::person::previews::*;
use crate::team::{get_initiator, Team};

pub enum Action {
    Shoot,
    Create,
    Pass,
}

pub fn possession_loop(
    offense: &Vec<Person>,
    defense: &Vec<Person>,
    matchups: &Vec<Matchup>,
    qtr_time: u16,
) -> PossesionData {
    let shot_clock = if qtr_time > SHOT_CLOCK {
        println!("{qtr_time} Left on the shot_clock");
        qtr_time
    } else {
        SHOT_CLOCK
    };

    let mut posession = PossesionData::new();

    let off_previews: Vec<PlayerOffPreview> = offense.clone().off_previews();
    let def_previews: Vec<PlayerDefPreview> = defense.clone().def_previews();

    while posession.duration < shot_clock {
        let initiator = get_initiator(&off_previews);
        let initiator_data = &offense.get_player(&initiator);
    }

    posession
}

//So Based off ball handler ability to get open
pub fn run_play(
    open_ability: Vec<PlayerOffPreview>,
    def_ablity: Vec<PlayerDefPreview>,
    matchups: Vec<Matchup>,
    ball_handler: PlayerOffPreview,
) -> [f32; 5] {
    let mut openess = [0.0; 5];

    openess.iter().enumerate().for_each(|(i, val)| {});

    println!("Openess Array: {:?}", openess);

    openess
}

#[test]
fn play_test() {}
