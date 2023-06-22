use crate::person::PersonId;
use crate::person::previews::{OffPreview, DefPreview};
use crate::game_handle::calculate::Openess;

use crate::math::z_score;

pub trait WeightedRandom {
    fn random_person(&self)-> PersonId;
}

impl WeightedRandom for Vec<OffPreview> {
    fn random_person(&self)-> PersonId {
        let mut ids: Vec<PersonId> = vec![];
        let mut val: Vec<f32> = vec![];

        self.iter().for_each(|player| {
            ids.push(player.0.clone());
            val.push(player.1.clone());
        });

        val = z_score(&val);

        let ball_handler = random_choice::random_choice()
            .random_choice_f32(&ids, &val, 1)
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        ball_handler
    }
}

impl WeightedRandom for Vec<DefPreview> {
    fn random_person(&self)-> PersonId {
        let mut ids: Vec<PersonId> = vec![];
        let mut val: Vec<f32> = vec![];

        self.iter().for_each(|player| {
            ids.push(player.0.clone());
            val.push(player.1.clone());
        });

        val = z_score(&val);

        let ball_handler = random_choice::random_choice()
            .random_choice_f32(&ids, &val, 1)
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        ball_handler
    }
}

impl WeightedRandom for Vec<Openess> {
    fn random_person(&self) -> PersonId {
        let mut ids: Vec<PersonId> = vec![];
        let mut val: Vec<f32> = vec![];

        self.iter().for_each(|player| {
            ids.push(player.0.clone());
            val.push(player.1.clone());
        });

        val = z_score(&val);

        let player = random_choice::random_choice()
            .random_choice_f32(&ids, &val, 1)
            .first()
            .unwrap()
            .to_owned()
            .to_owned();

        player 
    }

}
