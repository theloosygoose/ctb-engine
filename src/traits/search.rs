use crate::person::{Person, PersonId};
use crate::person::previews::{OffPreview, DefPreview};
use crate::person::matchup::Matchup;
use crate::game_handle::calculate::Openess;

pub trait Searchable{
    type Player;

    fn get_player(&self, player: &PersonId) -> Self::Player;
}

impl Searchable for Vec<Person> {
    type Player = Person;

    fn get_player(&self, player: &PersonId) -> Person {
        let player_data = self
            .iter()
            .find(|person_data| &person_data.person_id == player);

        player_data.unwrap().to_owned()
    }
}

impl Searchable for Vec<OffPreview> {
    type Player = OffPreview;

    fn get_player(&self, player: &PersonId) -> OffPreview{
        let player_preview = &self.iter().find(|x| &x.0 == player);

        player_preview.unwrap().clone()
    }
}

impl Searchable for Vec<DefPreview> {
    type Player = DefPreview;

    fn get_player(&self, player: &PersonId) -> DefPreview{
        let player_preview = &self.iter().find(|x| &x.0 == player);

        player_preview.unwrap().clone()
    }

}

impl Searchable for Vec<Matchup> {
    type Player = Matchup;

    fn get_player(&self, player: &PersonId) -> Matchup {
        let matchup = &self.iter().find(|x| &x.0 == player);

        matchup.unwrap().clone()
    }

}

impl Searchable for Vec<Openess> {
    type Player = f32;

    fn get_player(&self, player: &PersonId) -> f32 {
        let openess = &self.iter().find(|x| &x.0 == player);

        openess.unwrap().clone().1
    }

}
