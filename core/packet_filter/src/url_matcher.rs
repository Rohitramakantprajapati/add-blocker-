use aho_corasick::AhoCorasick;

use crate::{Result, VoidBlockError};

pub struct UrlMatcher {
    automaton: AhoCorasick,
    patterns: Vec<String>,
}

impl UrlMatcher {
    pub fn new<I, S>(patterns: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let patterns: Vec<String> = patterns.into_iter().map(Into::into).collect();
        if patterns.is_empty() {
            return Err(VoidBlockError::Pattern("at least one pattern is required".to_string()));
        }
        let automaton = AhoCorasick::new(&patterns).map_err(|error| VoidBlockError::Pattern(error.to_string()))?;
        Ok(Self { automaton, patterns })
    }

    pub fn is_match(&self, url: &str) -> bool {
        self.automaton.find(url).is_some()
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }
}
