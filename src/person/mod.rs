pub mod previews;
pub mod functions;
pub mod ratings;
pub mod matchup;

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

