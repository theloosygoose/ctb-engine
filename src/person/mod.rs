use crate::math::Sortable;

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
pub struct PlayerOffPreview(pub PersonId, pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerDefPreview(pub PersonId, pub f32);

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
                    self.swap(i -1, i);
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
                    self.swap(i -1, i);
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
                    self.swap(i -1, i);
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
                    self.swap(i -1, i);
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
    let player_0 = PlayerOffPreview(PersonId("john991".to_string()), 44.2);
    let player_1 = PlayerOffPreview(PersonId("carl12345".to_string()), 50.0);
    let player_2 = PlayerOffPreview(PersonId("nate44".to_string()), 80.2);
    let player_3 = PlayerOffPreview(PersonId("tim332".to_string()), 12.2);
    let player_4 = PlayerOffPreview(PersonId("john991".to_string()), 66.2);

    let test_players = vec![player_0, player_1, player_2, player_3, player_4] ;
    
    let mut norm_sort = test_players.clone();
    norm_sort.sort();
    
    let mut rev_sort = test_players.clone();
    rev_sort.rev_sort();

    assert_eq!(norm_sort[0].1, 80.2);
    assert_eq!(rev_sort[0].1, 12.2);
}

