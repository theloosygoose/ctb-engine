use crate::math::Sortable;
use super::{Person, PersonId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OffVal {
    Initiator,
    Driving,
    OffBall,
    FloorSpacing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefVal {
    OffBall,
    OnBall,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefPreview(pub PersonId, pub f32, pub DefVal, pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct OffPreview(pub PersonId, pub f32, pub OffVal, pub u16);

impl OffPreview {
    pub fn off_previews(off_team: &Vec<Person>, offense_val: OffVal) -> Vec<OffPreview> {
        let mut previews: Vec<OffPreview> = vec![];

        off_team.iter().for_each(|player| {
            previews.push(player.off_preview(offense_val));
        });
        previews
    }
}

impl DefPreview {
    pub fn def_previews(def_team: &Vec<Person>, defense_val: DefVal) -> Vec<DefPreview> {

        let mut previews: Vec<DefPreview> = vec![];
        def_team.iter().for_each(|player|{
            previews.push(player.def_preview(defense_val));
        });
        previews
    }
}

impl Sortable for Vec<OffPreview> {
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

impl Sortable for Vec<DefPreview> {
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

pub trait RandomPerson {
    fn pick_random_val(&self)-> PersonId;
}

impl RandomPerson for Vec<OffPreview> {
    fn pick_random_val(&self)-> PersonId {
        let mut ids: Vec<PersonId> = vec![];
        let mut val: Vec<f32> = vec![];

        &self.iter().for_each(|player| {
            ids.push(player.0.clone());
            val.push(player.1.clone());
        });

        let ball_handler = random_choice::random_choice()
            .random_choice_f32(&ids, &val, 1)
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        ball_handler
    }
}
