use super::{Person, PersonId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OffVal {
    Initiator,
    Creation,
    OffBall,
    FloorSpacing,
    Passing,
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
        previews.sort_by_key(|k| k.3);
        previews
    }
}

impl DefPreview {
    pub fn def_previews(def_team: &Vec<Person>, defense_val: DefVal) -> Vec<DefPreview> {

        let mut previews: Vec<DefPreview> = vec![];
        def_team.iter().for_each(|player|{
            previews.push(player.def_preview(defense_val));
        });
        previews.sort_by_key(|k| k.3);
        previews
    }
}


