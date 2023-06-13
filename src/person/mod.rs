pub mod previews;
pub mod functions;
pub mod ratings;
pub mod matchup;

use crate::weights::*;
use self::previews::{OffVal, OffPreview, DefVal, DefPreview};

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
    pub fn off_preview(&self, value_type: OffVal) -> OffPreview {
        let rtngs = self.intangibles;
        let personality = self.personality;

        match value_type {
            OffVal::Initiator => {
                let value = ((rtngs.ball_handle as f32 * OFF_ABILITY[0])
                         + (rtngs.off_awareness as f32 * OFF_ABILITY[1])
                         + (rtngs.touch as f32 * OFF_ABILITY[2])
                         + (rtngs.speed as f32 * OFF_ABILITY[2])
                         + (rtngs.burst as f32 * OFF_ABILITY[3])
                         + (personality.intelligence as f32 * OFF_ABILITY[4])
                         + (personality.dog as f32 * OFF_ABILITY[5]))
                    / (7.0);

                OffPreview(self.person_id.clone(), value, value_type)
            },
            OffVal::OffBall => {

            },
            OffVal::Driving => {

            },
            OffVal::FloorSpacing => {

            },


        }

    pub fn def_preview(&self, value_type: DefVal) -> DefPreview {
        let rtngs = self.intangibles;
        let personality = self.personality;

        match value_type {
            DefVal::OffBall => {

            },
            DefVal::OnBall => {

            },


        }

    }

    }
}
