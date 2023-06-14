use crate::person::{Person, PersonId};
use crate::person::previews::*;
// use crate::person::matchup::*;
use random_choice;

#[derive(Debug)]
pub struct Matchup(PersonId, PersonId, f32);

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

pub trait Team {
    fn off_previews(self, offense_val: OffVal) -> Vec<OffPreview>;

    fn def_previews(self, defense_val: DefVal) -> Vec<DefPreview>;

    fn get_player(&self, player: &PersonId) -> Person;
}

impl Team for Vec<Person> {
    fn off_previews(self, offense_val: OffVal) -> Vec<OffPreview> {
        let mut previews: Vec<OffPreview> = vec![];

        self.iter().for_each(|player| {

            previews.push(player.off_preview(offense_val));
        });

        previews
    }

    fn def_previews(self, defense_val: DefVal) -> Vec<DefPreview> {
        let mut previews: Vec<DefPreview> = vec![];

        self.iter().for_each(|player|{
            previews.push(player.def_preview(defense_val));
        });

        previews
    }

    fn get_player(&self, player: &PersonId) -> Person {
        let player_data = self
            .iter()
            .find(|person_data| &person_data.person_id == player);

        player_data.unwrap().clone()
    }

}
