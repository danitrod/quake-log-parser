use serde::Serialize;
use std::collections::HashMap;

const WORLD_NAME: &str = "<world>";

#[derive(Serialize)]
pub struct QuakeGameSummary {
    total_kills: usize,
    players: Vec<String>,
    kills: HashMap<String, usize>,
    #[serde(rename = "kills_by_means")]
    death_causes: HashMap<String, usize>,
}

impl QuakeGameSummary {
    pub fn new() -> Self {
        Self {
            total_kills: 0,
            players: Vec::new(),
            kills: HashMap::new(),
            death_causes: HashMap::new(),
        }
    }

    pub fn process_kill(
        &mut self,
        killer_name: String,
        killed_name: String,
        death_cause: String,
    ) -> Result<(), String> {
        self.total_kills += 1;

        if !self.players.contains(&killed_name) {
            return Err(format!(
                "error processing kill: player {killed_name} is not connected"
            ));
        }

        self.death_causes
            .entry(death_cause)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if &killer_name == WORLD_NAME {
            // If player was killed by the world, it loses a kill score.
            self.kills.entry(killed_name.clone()).and_modify(|count| {
                if *count > 0 {
                    *count -= 1;
                }
            });
            return Ok(());
        }

        if !self.players.contains(&killer_name) {
            return Err(format!(
                "error processing kill: player {killer_name} is not connected"
            ));
        }

        self.kills
            .entry(killer_name)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        Ok(())
    }

    pub fn add_player(&mut self, player_name: String) {
        if !self.players.contains(&player_name) {
            self.players.push(player_name.clone());
            self.kills.insert(player_name, 0);
        }
    }
}

#[test]
fn should_fail_to_process_kill_from_inexistent_player() {
    let mut summary = QuakeGameSummary::new();
    let result = summary.process_kill(
        "killer".to_string(),
        "killed".to_string(),
        "death_cause".to_string(),
    );

    assert!(result.is_err());
}

#[test]
fn should_add_player() {
    let mut summary = QuakeGameSummary::new();
    summary.add_player("player1".to_string());

    assert_eq!(summary.players, vec!["player1".to_string()]);
    assert_eq!(summary.kills["player1"], 0);
}

#[test]
fn should_decrease_kill_count_if_player_dies_from_world() {
    let mut summary = QuakeGameSummary::new();
    summary.add_player("player1".to_string());
    summary.add_player("player2".to_string());
    let _ = summary.process_kill(
        "player1".to_string(),
        "player2".to_string(),
        "cause1".to_string(),
    );
    assert_eq!(summary.kills["player1"], 1);
    let _ = summary.process_kill(
        "<world>".to_string(),
        "player1".to_string(),
        "cause1".to_string(),
    );
    assert_eq!(summary.kills["player1"], 0);
}
