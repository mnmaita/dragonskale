use bevy::{
    app::Update,
    ecs::{
        event::{Event, EventReader},
        system::Query,
    },
    prelude::*,
};

use crate::playing;

use super::Player;

pub(super) struct ScoreSystemPlugin;

impl Plugin for ScoreSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreEvent>();
        app.add_systems(Update, update_player_score.run_if(playing()));
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Score {
    current: i32,
    multiplier: i16,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            current: 0,
            multiplier: 1,
        }
    }
}

impl Score {
    pub fn new(current: i32, multiplier: i16) -> Self {
        Self {
            current,
            multiplier,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn multiplier(&self) -> i16 {
        self.multiplier
    }

    pub fn increase_multiplier_by_one(&mut self) {
        self.multiplier += 1;
    }

    pub fn reset_multiplier(&mut self) {
        self.multiplier = 1;
    }

    pub fn add_with_multiplier(&mut self, value: i32) {
        self.current += value * self.multiplier as i32;
    }
}

#[derive(Event)]
pub struct ScoreEvent {
    points: i32,
    score_event_type: ScoreEventType,
}
#[derive(Component)]
pub enum ScoreEventType {
    AddPoints,
    ResetMultiplier,
}

impl ScoreEvent {
    pub fn new(points: i32, score_event_type: ScoreEventType) -> Self {
        Self {
            points,
            score_event_type,
        }
    }
}

fn update_player_score(
    mut score_event_reader: EventReader<ScoreEvent>,
    mut player_score_query: Query<&mut Score, With<Player>>,
) {
    for ScoreEvent {
        points,
        score_event_type,
    } in score_event_reader.read()
    {
        let Ok(mut score_system) = player_score_query.get_single_mut() else {
            return;
        };

        match score_event_type {
            ScoreEventType::AddPoints => {
                score_system.add_with_multiplier(*points);
                score_system.increase_multiplier_by_one();
            }
            ScoreEventType::ResetMultiplier => {
                score_system.reset_multiplier();
            }
        }
    }
}
