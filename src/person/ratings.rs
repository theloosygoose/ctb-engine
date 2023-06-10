#[derive(Debug, Clone, Copy)]
pub struct IntangibleRatings {
    pub strength: u16,
    pub fluidity: u16,
    pub burst: u16,
    pub speed: u16,
    pub height: u16,
    pub wingspan: u16,

    pub off_awareness: u16,
    pub def_awareness: u16,
    pub shot_form: u16,
    pub touch: u16,
    pub pass_accuracy: u16,
    pub ball_handle: u16,
    pub lateral: u16,
    pub hands: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Personality {
    pub work_ethic: u16,
    pub intelligence: u16,
    pub creativity: u16,
    pub adaptability: u16,
    pub loyalty: u16,
    pub dog: u16,
}
