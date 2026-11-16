// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-display-config: display config component model.
//!
//! Real implementation: configs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayConfigError {
    EmptyConfigName,
    DuplicateConfig(String),
    EmptyEventSubject,
}

impl fmt::Display for DisplayConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyConfigName => write!(f, "config name must not be empty"),
            Self::DuplicateConfig(id) => write!(f, "duplicate config {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DisplayConfigError {}

pub struct DisplayConfig {
    pub name: String,
    configs: BTreeMap<String, Config>,
    events: BTreeMap<String, Event>,
}

impl DisplayConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            configs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_config(&mut self, item: Config) -> Result<(), DisplayConfigError> {
        if item.name.trim().is_empty() {
            return Err(DisplayConfigError::EmptyConfigName);
        }
        let id = item.id.clone();
        if self.configs.contains_key(&id) {
            return Err(DisplayConfigError::DuplicateConfig(id.clone()));
        }
        self.configs.insert(id, item);
        Ok(())
    }

    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    pub fn get_config(&self, id: &str) -> Option<&Config> {
        self.configs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DisplayConfigError> {
        if event.subject.trim().is_empty() {
            return Err(DisplayConfigError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config(id: &str, name: &str) -> Config {
        Config {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DisplayConfig::new("DisplayConfig");
        assert_eq!(m.name, "DisplayConfig");
        assert_eq!(m.config_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_config() {
        let mut m = DisplayConfig::new("DisplayConfig");
        m.add_config(sample_config("a1", "Alpha")).unwrap();
        assert_eq!(m.config_count(), 1);
        assert!(m.get_config("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DisplayConfig::new("DisplayConfig");
        assert!(m.add_config(sample_config("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DisplayConfig::new("DisplayConfig");
        m.add_config(sample_config("x", "One")).unwrap();
        assert!(m.add_config(sample_config("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DisplayConfig::new("DisplayConfig");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
