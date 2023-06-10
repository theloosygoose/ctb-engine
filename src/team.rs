use crate::person::{PersonId,PlayerOffPreview};
use random_choice;

#[derive(Debug)]
pub struct Matchup(PersonId, PersonId, f32);

pub fn get_ballhander(off_team_usg: &Vec<PlayerOffPreview>) -> PersonId {
    let mut ids: Vec<PersonId> = vec![];
    let mut val: Vec<f32> = vec![];

    off_team_usg.iter().for_each(|player| {
        ids.push(player.0.clone());
        val.push(player.1.clone());
    });

    let ball_handler = random_choice::random_choice()
        .random_choice_f32(&ids, &val, 1)
        .first()
        .unwrap()
        .to_owned()
        .to_owned();

    ball_handler
}
