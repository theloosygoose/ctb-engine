use crate::person::{Person, PersonId, Matchup, PlayerOffPreview, PlayerDefPreview};
use crate::team::{get_initiator, Team};
use super::possession::*;
use super::constants::*;

pub enum Action {
    
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

    let off_previews:Vec<PlayerOffPreview> = offense.clone().off_previews(); 
    let def_previews:Vec<PlayerDefPreview> = defense.clone().def_previews();

    while posession.duration < shot_clock {
        let initiator = get_initiator(&off_previews);
        let initiator_data = &offense.get_player(&initiator);
    };
    
    posession
}


//So Based off ball handler ability to get open 
pub fn run_play(
    open_ability: Vec<PlayerOffPreview>, 
    def_ablity: Vec<PlayerDefPreview>,
    ball_handler: PlayerOffPreview,
    ) -> [f32; 4] {
    
    let openess = [0.0, 0.0, 0.0, 0.0];
    
    openess
}

