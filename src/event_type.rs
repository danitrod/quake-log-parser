pub enum EventType {
    InitGame,
    ClientUserinfoChanged,
    Kill,
    Unknown,
}

impl EventType {
    pub fn parse_from_string(event_name: &str) -> Self {
        if event_name.len() == 0 {
            return EventType::Unknown;
        };

        match &event_name[0..event_name.len() - 1] {
            "InitGame" => EventType::InitGame,
            "Kill" => EventType::Kill,
            "ClientUserinfoChanged" => EventType::ClientUserinfoChanged,
            _ => EventType::Unknown,
        }
    }
}
