use super::PersonId;
use super::previews::*;

use crate::weights::{HEIGHT_MATCHUP, RATING_MATCHUP};

#[derive(Debug)]
pub struct Matchup(pub PersonId, pub PersonId);

impl Matchup {
    pub fn gen(offense_creation:&Vec<OffPreview>, on_ball_defense:&Vec<DefPreview>) -> Vec<Matchup> {
        let mut matchups: Vec<Matchup> = vec![];

        let mut off_abil: Vec<(PersonId, u16)> = vec![];
        let mut def_abil: Vec<(PersonId, u16)> = vec![];

        offense_creation.iter().for_each(|player|{
            let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;

            off_abil.push((player.0.clone(), value));
        });

        on_ball_defense.iter().for_each(|player|{
            let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;

            def_abil.push((player.0.clone(), value));
        });

        off_abil.sort_by_key(|k| k.1);
        def_abil.sort_by_key(|k| k.1);

        off_abil.iter().enumerate().for_each(|(i, off_player)|{

            matchups.push(Matchup(off_player.0.clone(), def_abil[i].0.clone()))
        });

        matchups
    }
}
