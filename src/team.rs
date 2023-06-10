use crate::person::{Person, PersonId};
use std::collections::HashMap;

pub trait Lineup {
   fn calculate_usg(&self) -> HashMap<PersonId, f32>;
}
