use crate::{event_type::EventType, report::QuakeGameSummary};
use serde_json;
use std::{collections::HashMap, str::Split};

pub struct QuakeLogParser {
    game_summaries: HashMap<String, QuakeGameSummary>,
    current_game: usize,
}

impl QuakeLogParser {
    pub fn new() -> Self {
        Self {
            game_summaries: HashMap::new(),
            current_game: 0,
        }
    }

    pub fn read_line(&mut self, line: String) -> Result<(), String> {
        let mut tokens = line.trim().split(" ");

        // First token is always the time
        tokens.next();

        let event_type = match tokens.next() {
            Some(token) => Ok(EventType::parse_from_string(token)),
            None => Err("line should have at least 2 tokens".to_string()),
        }?;

        match event_type {
            EventType::InitGame => {
                self.current_game += 1;
                self.game_summaries
                    .insert(self.current_game_id(), QuakeGameSummary::new());
                Ok(())
            }
            EventType::ClientUserinfoChanged => self.process_player_connected_event(tokens),
            EventType::Kill => self.process_kill_event(tokens),
            _ => Ok(()),
        }
    }

    fn process_player_connected_event(&mut self, mut tokens: Split<&str>) -> Result<(), String> {
        // Player connected event format:
        // 5 n\Dono da Bola\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
        // The first token is irrelevant.
        // Player name can be found within the n\ ant \t characters, in the next tokens.
        tokens.next();

        let mut player_name = tokens
            .next()
            .ok_or("error parsing user info event: player name not found")?
            .strip_prefix(r"n\")
            .ok_or("error parsing user info event: player name not in expected format")?
            .to_string();

        loop {
            let next_token = tokens.next();

            if !next_token.is_some() {
                player_name = player_name
                    .split(r"\t")
                    .next()
                    .ok_or("error parsing user info event: player name not in expected format")?
                    .to_string();
                break;
            }

            player_name.push_str(&format!(" {}", next_token.unwrap()));
        }

        self.game_summaries
            .get_mut(&self.current_game_id())
            .ok_or("could not find game report")?
            .add_player(player_name.to_string());

        Ok(())
    }

    fn process_kill_event(&mut self, mut tokens: Split<&str>) -> Result<(), String> {
        // Kill event format:
        // 2 3 7: Isgalamido killed Mocinha by MOD_ROCKET_SPLASH
        // The first three tokens are irrelevant.
        // Fourth is killer name.
        // Sixth is killed name.
        // Eigth is death cause.
        (0..3).for_each(|_| {
            tokens.next();
        });

        // Read killer player name and stopword 'killed'
        let killer_name = read_name(&mut tokens, "killed")?;

        // Read killed player name and stopword 'by'
        let killed_name = read_name(&mut tokens, "by")?;

        let death_cause = tokens
            .next()
            .ok_or("error parsing kill event: death cause not found")?;

        self.game_summaries
            .get_mut(&self.current_game_id())
            .ok_or("could not find current game report")?
            .process_kill(killer_name, killed_name, death_cause.to_string())?;

        Ok(())
    }

    fn current_game_id(&self) -> String {
        format!("game_{}", self.current_game)
    }

    pub fn output_report_json(&self) -> String {
        serde_json::to_string_pretty(&self.game_summaries)
            .expect("failed to serialize game report to JSON")
    }
}

fn read_name(tokens: &mut Split<&str>, stopword: &str) -> Result<String, &'static str> {
    // Names can have spaces. Read until the stopword
    let mut name = tokens
        .next()
        .ok_or("error parsing kill event: killer player name not found")?
        .to_string();

    loop {
        let token = tokens
            .next()
            .ok_or("error parsing kill event: killer player name not found")?;

        if token == stopword {
            break;
        }

        name.push_str(&format!(" {}", token));
    }

    Ok(name)
}

#[test]
fn should_read_name_until_stopword() {
    let name = "Player name stopword";
    let parsed_name = read_name(&mut name.split(" "), "stopword");

    assert!(parsed_name.is_ok());
    assert_eq!(parsed_name.unwrap(), "Player name");
}

#[test]
fn should_process_connected_player_event() {
    let mut p = QuakeLogParser::new();
    let game_init = r"0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0".to_string();
    let connected_player = r"21:51 ClientUserinfoChanged: 3 n\Dono da Bola\t\0\model\sarge/krusade\hmodel\sarge/krusade\g_redteam\\g_blueteam\\c1\5\c2\5\hc\95\w\0\l\0\tt\0\tl\0".to_string();

    let result = p.read_line(game_init);
    assert!(result.is_ok());
    let result = p.read_line(connected_player);
    assert!(result.is_ok());

    assert_eq!(p.current_game_id(), "game_1");
    assert!(p.output_report_json().contains("Dono da Bola"));
}

#[test]
fn should_process_kill_event() {
    let mut p = QuakeLogParser::new();
    let game_init = r"0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0".to_string();
    let connected_player_1 = r"20:38 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0".to_string();
    let connected_player_2 = r"21:53 ClientUserinfoChanged: 3 n\Mocinha\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0".to_string();
    let kill = r"22:06 Kill: 2 3 7: Isgalamido killed Mocinha by MOD_ROCKET_SPLASH".to_string();

    let result = p.read_line(game_init);
    assert!(result.is_ok());
    let result = p.read_line(connected_player_1);
    assert!(result.is_ok());
    let result = p.read_line(connected_player_2);
    assert!(result.is_ok());
    let result = p.read_line(kill);
    assert!(result.is_ok());

    assert_eq!(p.current_game_id(), "game_1");
    assert!(p.output_report_json().contains(r#""Isgalamido": 1"#));
    assert!(p.output_report_json().contains(r#""MOD_ROCKET_SPLASH": 1"#));
}
