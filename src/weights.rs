pub const INITIATOR: [f32; 9] = [
    1.5, //ball handle
    1.8, //off_awareness
    1.4, //pass_accuracy
    1.1, //touch 
    1.0, //shot_form
    1.0, //speed
    0.9, //burst
    1.6, //intelligence
    1.1, //creativity
];

pub const OFF_BALL: [f32; 5] = [
    1.5, //off_awareness
    1.5, //speed
    1.3, //burst
    1.4, //intelligence
    1.3, //dog
];

pub const DRIVING: [f32; 8] = [
    1.0, //off_awareness
    1.0, //touch
    1.3, //speed
    1.7, //burst
    1.4, //strength
    1.3, //fluidity
    0.9, //intelligence
    1.4, //dog
];

pub const SPACING: [f32; 4] = [
    1.0, //off_awareness
    1.7, //shot_form
    1.4, //touch
    1.0, //intelligence
];

pub const D_ONBALL: [f32; 8] = [
    1.0, //def_awareness
    1.7, //lateral
    1.4, //fluidity
    1.1, //burst
    1.0, //height
    1.0, //wingspan
    1.7, //dog
    1.0, //intelligence
];

pub const D_OFFBALL: [f32; 8] = [
    1.7, //def_awareness
    1.0, //lateral
    1.2, //speed
    1.1, //burst
    1.0, //height
    1.4, //wingspan
    1.3, //dog
    1.0, //intelligence
];

pub const HEIGHT_MATCHUP:u16 = 150;

pub const RATING_MATCHUP:f32 = 1.5;
