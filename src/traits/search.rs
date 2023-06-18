pub trait Searchable {
    fn get_player(&self, player: PersonId) -> PersonId;
}
