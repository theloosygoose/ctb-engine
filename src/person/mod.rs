pub mod previews;
pub mod functions;
pub mod ratings;
pub mod matchup;

use crate::weights::*;
use self::previews::*;

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

impl Person {
    pub fn off_ability(&self) -> PlayerOffPreview {
        let rtngs = self.intangibles;
        let personality = self.personality;

        let off_abilty = ((rtngs.ball_handle as f32 * OFF_ABILITY[0])
            + (rtngs.off_awareness as f32 * OFF_ABILITY[1])
            + (rtngs.touch as f32 * OFF_ABILITY[2])
            + (rtngs.speed as f32 * OFF_ABILITY[2])
            + (rtngs.burst as f32 * OFF_ABILITY[3])
            + (personality.intelligence as f32 * OFF_ABILITY[4])
            + (personality.dog as f32 * OFF_ABILITY[5]))
            / (7.0);

        PlayerOffPreview(self.person_id.clone(), off_abilty)
    }

    pub fn def_ability(&self) -> PlayerDefPreview {
        let rtngs = self.intangibles;
        let personality = self.personality;

        let def_ability = ((rtngs.lateral as f32 * DEF_ABILITY[0])
            + (rtngs.def_awareness as f32 * DEF_ABILITY[1])
            + (rtngs.speed as f32 * DEF_ABILITY[2])
            + (rtngs.strength as f32 * DEF_ABILITY[3])
            + (rtngs.burst as f32 * DEF_ABILITY[4])
            + (personality.intelligence as f32 * DEF_ABILITY[5])
            + (personality.dog as f32 * OFF_ABILITY[6]))
            / (7.0);

        PlayerDefPreview(self.person_id.clone(), def_ability)
    }
}

