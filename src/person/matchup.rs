use super::PersonId;

use super::previews::{OffPreview, DefPreview};

#[derive(Debug)]
pub struct Matchup(pub PersonId, pub PersonId);

impl Matchup {
    pub fn new(offense: &Vec<OffPreview>, defense: &Vec<DefPreview>) -> Vec<Matchup> {
        let mut matchups: Vec<Matchup> = vec![];
        let mut def_used: Vec<usize> = vec![];

        offense.iter().enumerate().for_each(|(i, o_player)|{
            
        });

        matchups
    }
}
