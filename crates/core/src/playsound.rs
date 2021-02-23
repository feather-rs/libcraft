//! Data sourced from: https://minecraft.gamepedia.com/Commands/playsound
// THIS MODULE NEEDS LOTS OF WORK

use super::positions;

struct Playsound {
    sound: Sounds,
    source: Option<Source>,           // music category
    targets: Option<String>,          // player name of ID
    pos: Option<positions::Position>, // idk how Position works; i need to look at it
    volume: Option<f32>,
    pitch: Option<f32>,
    min_volume: Option<f32>,
}

impl Playsound {
    pub fn new(
        sound: Sounds,
        source: Option<Source>,
        targets: Option<String>,
        pos: Option<positions::Position>,
        volume: Option<f32>,
        pitch: Option<f32>,
        min_volume: Option<f32>,
    ) -> Self {
        Self {
            sound,
            source,
            targets,
            pos,
            volume,
            pitch,
            min_volume,
        }
    }
}

pub enum Sounds {}

pub enum Source {
    Master,
    Music,
    Record,
    Weather,
    Block,
    Hostile,
    Neutral,
    Player,
    Ambient,
    Voice,
}
