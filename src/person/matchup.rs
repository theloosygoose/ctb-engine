use super::{PersonId, PlayerOffPreview, PlayerDefPreview};

#[derive(Debug)]
pub struct Matchup(pub PersonId, pub PersonId);

impl Matchup {
    pub fn gen(offense: &Vec<PlayerOffPreview>, defense: &Vec<PlayerDefPreview>) -> Vec<Matchup> {
        let mut matchups: Vec<Matchup> = vec![];

        let mut def_used: Vec<usize> = vec![];

        matchups
    }
}
