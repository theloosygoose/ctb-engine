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
pub struct Matchup(PersonId, PersonId);




#[derive(Debug, Clone, PartialEq)]
pub struct PlayerOffPreview(PersonId, f32);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerDefPreview(PersonId, f32);

impl PlayerDefPreview{
    pub fn from_tuple(tup: (PersonId, f32)) -> PlayerDefPreview {
        PlayerDefPreview(tup.0, tup.1)    
    }
}

impl PlayerOffPreview {
    pub fn from_tuple(tup: (PersonId, f32)) -> PlayerOffPreview {
        PlayerOffPreview(tup.0, tup.1)    
    }
    
}

