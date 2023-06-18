use crate::person::{Person, PersonId};
use crate::person::previews::*;
use random_choice;


pub fn pick_random_val(off_team_usg: &Vec<OffPreview>) -> PersonId {
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

pub trait Searchable {
    fn get_player(&self, player: &PersonId) -> Person;
}

impl Searchable for Vec<Person> {
    fn get_player(&self, player: &PersonId) -> Person {
        let player_data = self
            .iter()
            .find(|person_data| &person_data.person_id == player);

        player_data.unwrap().clone()
    }
}
