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
            OffVal::Passing=> {
                let value = ((rtngs.ball_handle as f32 * PASSING[0])
                             + (rtngs.off_awareness as f32 * PASSING[1])
                             + (rtngs.height as f32 * PASSING[2])
                             + (rtngs.pass_accuracy as f32 * PASSING[3])
                             + (rtngs.touch as f32 * PASSING[4])
                             + (personality.intelligence as f32 * PASSING[5])
                             + (personality.creativity as f32 * PASSING[6])) / PASSING.iter().sum::<f32>();

                return OffPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
            OffVal::Initiator => {
                let value = ((rtngs.ball_handle as f32 * INITIATOR[0])
                             + (rtngs.off_awareness as f32 * INITIATOR[1])
                             + (rtngs.pass_accuracy as f32 * INITIATOR[2])
                             + (rtngs.touch as f32 * INITIATOR[3])
                             + (rtngs.shot_form as f32 * INITIATOR[4])
                             + (rtngs.speed as f32 * INITIATOR[5])
                             + (rtngs.burst as f32 * INITIATOR[6])
                             + (personality.intelligence as f32 * INITIATOR[7])
                             + (personality.creativity as f32 * INITIATOR[8])) / INITIATOR.iter().sum::<f32>();

                return OffPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
            OffVal::OffBall => {
                let value = ((rtngs.off_awareness as f32 * OFF_BALL[0])
                             + (rtngs.speed as f32 * OFF_BALL[1])
                             + (rtngs.burst as f32 * OFF_BALL[2])
                             + (personality.intelligence as f32 * OFF_BALL[3])
                             + (personality.dog as f32 * OFF_BALL[4])) / OFF_BALL.iter().sum::<f32>();

                return OffPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
            OffVal::Creation => {
                let value = ((rtngs.off_awareness as f32 * CREATION[0])
                             + (rtngs.touch as f32 * CREATION[1])
                             + (rtngs.speed as f32 * CREATION[2])
                             + (rtngs.burst as f32 * CREATION[3])
                             + (rtngs.strength as f32 * CREATION[4])
                             + (rtngs.fluidity as f32 * CREATION[5])
                             + (personality.intelligence as f32 * CREATION[6])
                             + (personality.dog as f32 * CREATION[7])) / CREATION.iter().sum::<f32>();

                return OffPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
            OffVal::FloorSpacing => {
                let value = ((rtngs.off_awareness as f32 * SPACING[0])
                             + (rtngs.shot_form as f32 * SPACING[1])
                             + (rtngs.touch as f32 * SPACING[2])
                             + (personality.intelligence as f32 * SPACING[3])) / SPACING.iter().sum::<f32>();


                return OffPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
        }
    }

    pub fn def_preview(&self, value_type: DefVal) -> DefPreview {
        let rtngs = self.intangibles;
        let personality = self.personality;

        match value_type {
            DefVal::OffBall => {
                let value =    ((rtngs.def_awareness as f32 * D_OFFBALL[0])
                                + (rtngs.lateral as f32 * D_OFFBALL[1])
                                + (rtngs.speed as f32 * D_OFFBALL[2])
                                + (rtngs.burst as f32 * D_OFFBALL[3])
                                + (rtngs.wingspan as f32 * D_OFFBALL[4])
                                + (personality.dog as f32 * D_OFFBALL[5])
                                + (personality.intelligence as f32 * D_OFFBALL[6])) / (D_OFFBALL.iter().sum::<f32>());

                return DefPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            },
            DefVal::OnBall => {
                let value =    ((rtngs.def_awareness as f32 * D_ONBALL[0])
                                + ((u16::pow(rtngs.lateral, 2)/100) as f32 * D_ONBALL[1])
                                + (rtngs.fluidity as f32 * D_ONBALL[2])
                                + (rtngs.burst as f32 * D_ONBALL[3])
                                + (rtngs.wingspan as f32 * D_ONBALL[4])
                                + (personality.dog as f32 * D_ONBALL[5])
                                + (personality.intelligence as f32 * D_ONBALL[6])) / (D_ONBALL.iter().sum::<f32>());

                return DefPreview(self.person_id.clone(), value, value_type, self.intangibles.height);
            }
        }
    }
}
