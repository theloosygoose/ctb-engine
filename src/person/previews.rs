use super::{PersonId, Person};

use crate::math::Sortable;
use crate::weights::{DEF_ABILITY, OFF_ABILITY};

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerOffPreview(pub PersonId, pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerDefPreview(pub PersonId, pub f32);

impl Sortable for Vec<PlayerOffPreview> {
    fn sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 < self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }

    fn rev_sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 > self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }
}

impl Sortable for Vec<PlayerDefPreview> {
    fn sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 < self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }

    fn rev_sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 > self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }
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
