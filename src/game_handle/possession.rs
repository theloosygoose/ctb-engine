use crate::person::PersonId;

pub enum ShotType {
    Three(bool),
    MidRange(bool),
    Inside(bool),
    Layup(bool),
    Dunk(bool),
    None,
}

pub enum Foul {
    Shooting(PersonId, PersonId, u16),
    OnFloor(PersonId, PersonId),
    None,
}

pub enum Turnover {
    OutOfBounds(PersonId),
    OffensiveFoul(PersonId),
    Steal(PersonId, PersonId),
    ShotClock(PersonId),
    None,
}

pub struct PossesionData {
    pub ongoing: bool,
    pub shot_type: ShotType,
    pub shot_taker: Option<PersonId>,

    pub defender: Option<PersonId>,

    pub foul: Foul,
    pub points_scored: u16,

    pub passes: Vec<PersonId>,

    pub duration: u16,
    pub rebounder: Option<PersonId>,
    pub turnover: Turnover,
}

impl PossesionData {
    pub fn new() -> PossesionData {
        PossesionData {
            ongoing: true,
            shot_type: ShotType::None,
            foul: Foul::None,
            points_scored: 0,
            shot_taker: None,
            defender: None,
            duration: 0,
            turnover: Turnover::None,
            rebounder: None,
            passes: vec![],
        }
    }

    pub fn pass(&mut self, passer: PersonId, reciver: PersonId, duration: u16) {
        self.duration += duration;

        if self.passes.len() == 0 {
            self.passes.push(passer);
            self.passes.push(reciver);
        } else {
            self.passes.push(reciver);
        }
    }

    pub fn rebound(&mut self, rebounder: PersonId) {
        self.rebounder = Some(rebounder);

        //End Possesion
        self.ongoing = false;
    }

    pub fn shot(
        &mut self,
        shot_type: ShotType,
        shot_taker: PersonId,
        defender: PersonId,
        duration: u16,
    ) {
        self.duration += duration;
        self.shot_taker = Some(shot_taker);
        self.defender = Some(defender);

        match shot_type {
            ShotType::Three(val) => {
                if val {
                    self.points_scored = 3
                }
            }
            ShotType::MidRange(val) => {
                if val {
                    self.points_scored = 2
                }
            }
            ShotType::Layup(val) => {
                if val {
                    self.points_scored = 2
                }
            }
            ShotType::Dunk(val) => {
                if val {
                    self.points_scored = 2
                }
            }
            ShotType::Inside(val) => {
                if val {
                    self.points_scored = 2
                }
            }
            ShotType::None => self.points_scored = 0,
        }
        self.shot_type = shot_type;
    }

    pub fn turnover(&mut self, turnover: Turnover, duration: u16) {
        self.duration += duration;
        self.turnover = turnover;
    }

    pub fn foul(&mut self, foul: Foul) {
        self.points_scored += match foul {
            Foul::None => 0,
            Foul::OnFloor(_, _) => 0,
            Foul::Shooting(_, _, val) => val,
        };
        self.foul = foul;
    }
}
