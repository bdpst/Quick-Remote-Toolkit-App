use serde::{Deserialize, Serialize};
// .NET ToUpperInvariant uses one-to-one casing and preserves dotless i.
// Rust's full casing would turn ß into SS and merge different WPF favorite keys.
fn invariant_upper(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch == '\u{131}' {
                return ch;
            }
            let mut uppercase = ch.to_uppercase();
            match (uppercase.next(), uppercase.next()) {
                (Some(single), None) => single,
                _ => ch,
            }
        })
        .collect()
}
#[derive(Clone, Default)]
pub struct Client {
    pub number: i32,
    pub computer: String,
    pub ip: String,
    pub person: String,
    pub status: String,
    pub domain: String,
    pub last_checked: String,
}
impl Client {
    pub fn key(&self) -> String {
        let pc = self.computer.trim();
        let ip = self.ip.trim();
        if !pc.is_empty() && pc != "-" {
            format!("computer:{}", invariant_upper(pc))
        } else if !ip.is_empty() && ip != "-" {
            format!("ip:{}", invariant_upper(ip))
        } else {
            String::new()
        }
    }
    pub fn matches(&self, query: &str) -> bool {
        let q = invariant_upper(query.trim());
        [&self.computer, &self.ip, &self.person, &self.domain]
            .iter()
            .any(|v| invariant_upper(v).contains(&q))
    }
    pub fn target(&self) -> &str {
        if self.ip.trim().is_empty() || self.ip.trim() == "-" {
            &self.computer
        } else {
            &self.ip
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Settings {
    pub clients_csv_path: String,
    pub ping_timeout_ms: i32,
    pub parallel_ping_limit: i32,
    pub favorite_computers: Vec<String>,
    pub show_favorites_only: bool,
    pub is_dark_theme: bool,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            clients_csv_path: String::new(),
            ping_timeout_ms: 1000,
            parallel_ping_limit: 24,
            favorite_computers: vec![],
            show_favorites_only: false,
            is_dark_theme: false,
            extra: Default::default(),
        }
    }
}
impl Settings {
    pub fn favorite(&self, c: &Client) -> bool {
        let key = invariant_upper(&c.key());
        self.favorite_computers
            .iter()
            .any(|x| !key.is_empty() && invariant_upper(x.trim()) == key)
    }
    pub fn toggle(&mut self, c: &Client) {
        if c.key().is_empty() {
            return;
        }
        if self.favorite(c) {
            let key = invariant_upper(&c.key());
            self.favorite_computers
                .retain(|x| invariant_upper(x.trim()) != key);
        } else {
            self.favorite_computers.push(c.key());
            self.favorite_computers
                .sort_unstable_by_key(|s| invariant_upper(s));
        }
    }
}
pub struct Log {
    pub time: String,
    pub target: String,
    pub action: String,
    pub result: String,
}
pub enum Event {
    Log(String, String, String),
    Error(String, String, String),
    Probe(u64, usize, String, String),
    ProbeStarted(u64, usize, String),
    ProbeDone(u64),
    Done(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn favorites_are_case_insensitive_and_shared_by_computer() {
        let mut s = Settings::default();
        let c = Client {
            computer: " PC-TEST ".into(),
            ..Default::default()
        };
        let alias = Client {
            computer: "pc-test".into(),
            ..Default::default()
        };
        s.toggle(&c);
        assert!(s.favorite(&alias));
        s.toggle(&alias);
        assert!(!s.favorite(&c));
        let russian = Client {
            computer: "пк-тест".into(),
            ..Default::default()
        };
        s.favorite_computers.push("COMPUTER:пк-тест".into());
        assert!(s.favorite(&russian));
        s.toggle(&russian);
        assert!(s.favorite_computers.is_empty());
        assert_eq!(invariant_upper("ßﬃıiİσςёǰŉ"), "ßﬃıIİΣΣЁǰŉ");
        let sharp_s = Client {
            computer: "PC-ß".into(),
            ..Default::default()
        };
        s.toggle(&sharp_s);
        assert_eq!(s.favorite_computers, ["computer:PC-ß"]);
        assert!(!s.favorite(&Client {
            computer: "PC-SS".into(),
            ..Default::default()
        }));
        assert!(!sharp_s.matches("PC-SS"));
        let dotted = Client {
            person: "İı".into(),
            ..Default::default()
        };
        assert!(!dotted.matches("i"));
    }
    #[test]
    fn settings_import_uses_wpf_names_and_defaults() {
        let s: Settings =
            serde_json::from_str(r#"{"IsDarkTheme":true,"FavoriteComputers":["PC-TEST"]}"#)
                .unwrap();
        assert!(s.is_dark_theme);
        assert_eq!(s.parallel_ping_limit, 24);
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("FavoriteComputers"));
        let legacy: Settings =
            serde_json::from_str(r#"{"PingTimeoutMs":-1,"ParallelPingLimit":-8}"#).unwrap();
        let roundtrip: Settings =
            serde_json::from_str(&serde_json::to_string(&legacy).unwrap()).unwrap();
        assert_eq!(roundtrip.ping_timeout_ms, -1);
        assert_eq!(roundtrip.parallel_ping_limit, -8);
    }
}

#[cfg(test)]
mod parity_tests {
    use super::*;
    #[test]
    fn wpf_favorite_keys_and_fallback() {
        let c = Client {
            computer: " pc-1 ".into(),
            ..Default::default()
        };
        let s: Settings =
            serde_json::from_str(r#"{"FavoriteComputers":["computer:PC-1"],"FutureSetting":17}"#)
                .unwrap();
        assert!(s.favorite(&c));
        assert_eq!(c.key(), "computer:PC-1");
        assert!(serde_json::to_string(&s).unwrap().contains("FutureSetting"));
        let ip = Client {
            computer: "-".into(),
            ip: "192.0.2.1".into(),
            ..Default::default()
        };
        assert_eq!(ip.key(), "ip:192.0.2.1");
        assert_eq!(ip.target(), "192.0.2.1");
        let mut s = Settings::default();
        s.toggle(&Client::default());
        assert!(s.favorite_computers.is_empty());
    }
    #[test]
    fn domain_search_and_trim() {
        let c = Client {
            domain: "EXAMPLE".into(),
            ..Default::default()
        };
        assert!(c.matches(" example "));
        assert!(!c.matches("other"));
    }
    #[test]
    fn dash_ip_uses_computer() {
        let c = Client {
            computer: "PC-1".into(),
            ip: "-".into(),
            ..Default::default()
        };
        assert_eq!(c.target(), "PC-1");
    }
}

// WPF MaxLength counts UTF-16 code units, including surrogate pairs.
pub fn truncate_utf16(text: &mut String, limit: usize) {
    let mut units = 0;
    let mut end = text.len();
    for (index, c) in text.char_indices() {
        units += c.len_utf16();
        if units > limit {
            end = index;
            break;
        }
    }
    text.truncate(end);
}
#[cfg(test)]
mod utf16_tests {
    #[test]
    fn limits_preserve_unicode_boundaries() {
        let mut s = "Я😀X".to_string();
        super::truncate_utf16(&mut s, 3);
        assert_eq!(s, "Я😀");
        super::truncate_utf16(&mut s, 2);
        assert_eq!(s, "Я");
    }
}

#[derive(Debug)]
pub struct PingReply {
    pub online: bool,
    pub round_trip_ms: Option<u32>,
    pub diagnostic: String,
}
impl PingReply {
    pub fn success(ms: u32) -> Self {
        Self {
            online: true,
            round_trip_ms: Some(ms),
            diagnostic: String::new(),
        }
    }
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            online: false,
            round_trip_ms: None,
            diagnostic: message.into(),
        }
    }
}
