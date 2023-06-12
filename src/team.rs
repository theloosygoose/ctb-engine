use crate::person::{Person, PersonId, PlayerOffPreview, PlayerDefPreview};
use random_choice;

#[derive(Debug)]
pub struct Matchup(PersonId, PersonId, f32);

pub fn get_initiator(off_team_usg: &Vec<PlayerOffPreview>) -> PersonId {
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
    fn off_previews(self) -> Vec<PlayerOffPreview>;
    
    fn def_previews(self) -> Vec<PlayerDefPreview>;

    fn get_player(&self, player: &PersonId) -> Person;

    fn gen_def_matchups(&self, off_team: Vec<Person>) -> Vec<Matchup>;
}

impl Team for Vec<Person> {
    fn off_previews(self) -> Vec<PlayerOffPreview> {
        
        let mut previews:Vec<PlayerOffPreview> = vec![];
        
        self.iter().for_each(|player| {
            previews.push(player.off_ability())
        });
        previews
    }
    
    fn def_previews(self) -> Vec<PlayerDefPreview> {
        let mut previews:Vec<PlayerDefPreview> = vec![];
        
        self.iter().for_each(|player| {
            previews.push(player.def_ability())
        });

        previews
    }
    
    fn get_player(&self, player: &PersonId) -> Person {
        let player_data = self.iter().find(|person_data| {
           &person_data.person_id == player
        });
        
        player_data.unwrap().clone()
    }

    fn gen_def_matchups(&self, off_team: Vec<Person>) -> Vec<Matchup> {
        off_team.iter().for_each({|player| {

        }})
        
    }
}
