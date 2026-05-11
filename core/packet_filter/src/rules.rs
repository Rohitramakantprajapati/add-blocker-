use std::path::Path;

use rusqlite::Connection;

use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRule {
    pub domain: String,
    pub selector: Option<String>,
}

pub fn load_rules(db_path: &Path) -> Result<Vec<BlockRule>> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut statement = conn.prepare("SELECT domain, COALESCE(selector, '') FROM blocked ORDER BY domain")?;
    let mut rows = statement.query([])?;
    let mut rules = Vec::new();

    while let Some(row) = rows.next()? {
        let selector_text: String = row.get(1)?;
        let selector = if selector_text.is_empty() { None } else { Some(selector_text) };
        rules.push(BlockRule {
            domain: row.get::<_, String>(0)?,
            selector,
        });
    }

    Ok(rules)
}
