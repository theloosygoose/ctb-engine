use super::{Person, PersonId, PlayerDefPreview, PlayerOffPreview};
use crate::math::Sortable;
use crate::weights::{DEF_ABILITY, OFF_ABILITY};

impl PlayerDefPreview {
    pub fn from_tuple(tup: (PersonId, f32)) -> PlayerDefPreview {
        PlayerDefPreview(tup.0, tup.1)
    }

    pub fn to_tuple(&mut self) -> (PersonId, f32) {
        let id = self.0.clone();
        let val = self.1.clone();

        (id, val)
    }
}

impl PlayerOffPreview {
    pub fn from_tuple(tup: (PersonId, f32)) -> PlayerOffPreview {
        PlayerOffPreview(tup.0, tup.1)
    }

    pub fn to_tuple(&mut self) -> (PersonId, f32) {
        let id = self.0.clone();
        let val = self.1.clone();

        (id, val)
    }
}

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

#[test]
fn sort_test() {
    let player_0 = PlayerOffPreview(PersonId("john991".to_string()), 44.2);
    let player_1 = PlayerOffPreview(PersonId("carl12345".to_string()), 50.0);
    let player_2 = PlayerOffPreview(PersonId("nate44".to_string()), 80.2);
    let player_3 = PlayerOffPreview(PersonId("tim332".to_string()), 12.2);
    let player_4 = PlayerOffPreview(PersonId("john991".to_string()), 66.2);

    let test_players = vec![player_0, player_1, player_2, player_3, player_4];

    let mut norm_sort = test_players.clone();
    norm_sort.sort();

    let mut rev_sort = test_players.clone();
    rev_sort.rev_sort();

    assert_eq!(norm_sort[0].1, 80.2);
    assert_eq!(rev_sort[0].1, 12.2);
}
