use crate::person::{Person, PersonId};

pub struct Lineup {
    offensive_rating: u16,
    defensive_rating: u16,
    pub players: Vec<PersonId>,
}



impl Lineup {

    fn calculate_ratings(&mut self){
        todo!();
    } 

    pub fn new(players: &Vec<Person>) -> Lineup {
        todo!();
    }

    pub fn subsitute(&mut self, sub_in: PersonId, sub_out: PersonId) {
        todo!();
    }
}
