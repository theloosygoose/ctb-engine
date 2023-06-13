use crate::math::Sortable;

use super::PersonId;


#[derive(Debug, Clone, PartialEq)]
pub enum PreviewType {
    Initiator,
    ScoringAbility,
    PlayAbility,
    OffBallAbility,
    DefOffBall,
    DefOnBall,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerVal(pub PersonId, pub f32, pub PreviewType);

impl Sortable for Vec<PlayerVal> {
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


#[test]
fn sort_test() {
    let player_0 = PlayerVal(PersonId("john991".to_string()), 44.2);
    let player_1 = PlayerVal(PersonId("carl12345".to_string()), 50.0);
    let player_2 = PlayerVal(PersonId("nate44".to_string()), 80.2);
    let player_3 = PlayerVal(PersonId("tim332".to_string()), 12.2);
    let player_4 = PlayerVal(PersonId("john991".to_string()), 66.2);

    let test_players = vec![player_0, player_1, player_2, player_3, player_4];

    let mut norm_sort = test_players.clone();
    norm_sort.sort();

    let mut rev_sort = test_players.clone();
    rev_sort.rev_sort();

    assert_eq!(norm_sort[0].1, 80.2);
    assert_eq!(rev_sort[0].1, 12.2);
}
