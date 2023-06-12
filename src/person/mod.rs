pub mod functions;
pub mod ratings;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PersonId(pub String);

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub person_id: PersonId,
    pub age: u16,
    pub team: Option<String>,

    pub personality: ratings::Personality,
    pub intangibles: ratings::IntangibleRatings,
}

#[derive(Debug)]
pub struct Matchup(pub PersonId, pub PersonId);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerOffPreview(pub PersonId, pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerDefPreview(pub PersonId, pub f32);

impl Matchup {
    pub fn gen(offense: &Vec<PlayerOffPreview>, defense: &Vec<PlayerDefPreview>) -> Vec<Matchup> {
        let mut matchups: Vec<Matchup> = vec![];

        let mut def_used: Vec<usize> = vec![];
        offense.iter().enumerate().for_each(|(i, off_player)| {

        });

        matchups
    }

}

