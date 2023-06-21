pub const PASSING: [f32; 7] = [
    1.0, //ball handle
    7.0, //off_awareness
    1.0, //height
    4.0, //pass_accuracy
    2.0, //touch 
    5.0, //intelligence
    4.0, //creativity
];

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

pub const CREATION: [f32; 8] = [
    2.0, //off_awareness
    1.5, //touch
    2.0, //speed
    5.0, //burst
    3.0, //strength
    2.0, //fluidity
    1.0, //intelligence
    2.0, //dog
];

pub const SPACING: [f32; 4] = [
    1.0, //off_awareness
    1.7, //shot_form
    1.4, //touch
    1.0, //intelligence
];

pub const D_ONBALL: [f32; 7] = [
    1.0, //def_awareness
    5.0, //lateral
    3.0, //fluidity
    1.0, //burst
    2.0, //wingspan
    2.0, //dog
    1.5, //intelligence
];

pub const D_OFFBALL: [f32; 7] = [
    1.7, //def_awareness
    1.0, //lateral
    1.2, //speed
    1.1, //burst
    1.4, //wingspan
    1.3, //dog
    1.0, //intelligence
];

pub const HEIGHT_MATCHUP:f32 = 1.104;
pub const HEIGHT_SMOOTHING_MATCHUP:u16 = 20;

pub const RATING_MATCHUP:f32 = 1.3;
