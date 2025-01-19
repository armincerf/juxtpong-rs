use bevy::prelude::*;

use super::resources::Score;
use crate::components::Player;
use crate::game::GameEvents;


#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score(mut commands: Commands) {
  // Player 1 score (top left)
  commands.spawn((
    Text(format!("0000")),
    Player::Player1,
    ScoreText,
  ));

  // Player 2 score (top right)
  commands.spawn((
    Text(format!("0")),
    Player::Player2,
    ScoreText,
  ));
}

pub fn score(
  mut events: EventReader<GameEvents>,
  mut score_text: Query<(&mut Text, &Player), With<ScoreText>>,
  mut score: ResMut<Score>,
) {
  for event in events.read() {
    match event {
      GameEvents::GainPoint(player) => {
        *score.0.entry(*player).or_default() += 1;
        let score = score.0.get(player).cloned().unwrap_or(0);
        for (mut text, owner) in &mut score_text {
          if owner != player {
            continue;
          }
          *text = Text(score.to_string());
          break;
        }
      }
      GameEvents::ResetScore => {
        score.0.clear();
        for (mut text, _) in &mut score_text {
          *text = Text("0".to_string());
        }
      }
      _ => {}
    }
  }
}
