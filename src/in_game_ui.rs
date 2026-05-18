//! Heads-up display: score counter and game-state label for in-game UI.

use bevy::prelude::*;

use crate::GameSystems;
use crate::ScreenState;
use crate::game_logic::{MoveCount, LifeCount, Score};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::InGame), setup_in_game_ui)
            .add_systems(
                Update,
                (update_score_text
                    .run_if(resource_changed::<Score>)
                    .in_set(GameSystems::AudioVisual),
                 update_life_count_text
                    .run_if(resource_changed::<LifeCount>)
                    .in_set(GameSystems::AudioVisual),
                 update_move_count_text
                     .run_if(resource_changed::<MoveCount>)
                     .in_set(GameSystems::AudioVisual),
                )
            );
    }
}

const SCORE_FONT_SIZE: f32 = 28.0;
const SCORE_TOP: f32 = 14.0;
const SCORE_LEFT: f32 = 16.0;

const COUNT_LEFT: f32 = 16.0;

const LIFE_LEFT: f32 = 200.0;

#[derive(Component, Default, Clone)]
struct ScoreText;

#[derive(Component, Default, Clone)]
struct MoveCountText;

#[derive(Component, Default, Clone)]
struct LifeCountText;

fn update_move_count_text(
    moves: Res<MoveCount>,
    mut text: Single<&mut Text, With<MoveCountText>>,
) {
    text.0 = moves.to_string();
}


fn update_life_count_text(
    moves: Res<LifeCount>,
    mut text: Single<&mut Text, With<LifeCountText>>,
) {
    text.0 = moves.to_string();
}

fn setup_in_game_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        ScoreText
        Text({Score::default().to_string()})
        TextFont { font_size: px(SCORE_FONT_SIZE) }
        TextColor(Color::WHITE)
        Node {
            position_type: PositionType::Absolute,
            top: px(SCORE_TOP),
            left: px(SCORE_LEFT),
        }
        DespawnOnExit::<ScreenState>(ScreenState::InGame)
    });

    commands.spawn_scene(bsn! {
        LifeCountText
        Text({LifeCount::default().to_string()})
        TextFont { font_size: px(SCORE_FONT_SIZE) }
        TextColor(Color::WHITE)
        Node {
            position_type: PositionType::Absolute,
            top: px(SCORE_TOP),
            right: px(LIFE_LEFT),
        }
        DespawnOnExit::<ScreenState>(ScreenState::InGame)
    });

    commands.spawn_scene(bsn! {
        MoveCountText
        Text({MoveCount::default().to_string()})
        TextFont { font_size: px(SCORE_FONT_SIZE) }
        TextColor(Color::WHITE)
        Node {
            position_type: PositionType::Absolute,
            top: px(SCORE_TOP),
            right: px(COUNT_LEFT),
        }
        DespawnOnExit::<ScreenState>(ScreenState::InGame)
    });
}

fn update_score_text(score: Res<Score>, mut text: Single<&mut Text, With<ScoreText>>) {
    text.0 = score.to_string();
}