#![forbid(unsafe_code)]

use jsentinel_events::{
    mock_demo_events, AccessEvent, EventId, EventKind, EventSeverity, EventSource, EventTimestamp,
    ProcessRef,
};
use rusqlite::types::Type;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageMode {
    LocalOnly,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabasePlan {
    pub mode: StorageMode,
    pub stores_events: bool,
    pub stores_user_identifiers: bool,
    pub notes: &'static str,
}

pub fn v1_database_plan() -> DatabasePlan {
    DatabasePlan {
        mode: StorageMode::LocalOnly,
        stores_events: true,
        stores_user_identifiers: false,
        notes: "JSentinel stores event data locally with explicit retention controls planned.",
    }
}

#[derive(Debug)]
pub enum DbError {
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    EventParse(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "sqlite error: {error}"),
            Self::Json(error) => write!(formatter, "json error: {error}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::EventParse(error) => write!(formatter, "event parse error: {error}"),
        }
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuery {
    pub kind: Option<EventKind>,
    pub severity: Option<EventSeverity>,
    pub text: Option<String>,
    pub limit: Option<u32>,
}

impl Default for EventQuery {
    fn default() -> Self {
        Self {
            kind: None,
            severity: None,
            text: None,
            limit: Some(100),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_events: u64,
    pub warnings: u64,
    pub critical: u64,
    pub process_events: u64,
    pub network_events: u64,
    pub file_events: u64,
    pub startup_events: u64,
    pub device_access_events: u64,
    pub locked_file_events: u64,
    pub security_events: u64,
    pub latest_event_timestamp: Option<EventTimestamp>,
    pub demo_data_only: bool,
}

pub struct Database {
    conn: Connection,
}

pub fn init_db(path: impl AsRef<Path>) -> DbResult<Database> {
    Database::open(path)
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> DbResult<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        let database = Self { conn };
        database.init_schema()?;
        Ok(database)
    }

    fn init_schema(&self) -> DbResult<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                kind TEXT NOT NULL,
                severity TEXT NOT NULL,
                source TEXT NOT NULL,
                process_pid INTEGER NULL,
                process_name TEXT NULL,
                process_path TEXT NULL,
                title TEXT NOT NULL,
                summary TEXT NOT NULL,
                target TEXT NULL,
                metadata_json TEXT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS action_history (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                action_type TEXT NOT NULL,
                target TEXT NOT NULL,
                risk_level TEXT NOT NULL,
                result TEXT NOT NULL,
                error TEXT NULL
            );

            INSERT OR IGNORE INTO schema_migrations (version, applied_at)
            VALUES (1, datetime('now'));
            "#,
        )?;
        Ok(())
    }

    pub fn insert_event(&self, event: &AccessEvent) -> DbResult<()> {
        let metadata_json = event
            .metadata_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let process_pid = event
            .process
            .as_ref()
            .and_then(|process| process.pid)
            .map(i64::from);
        let process_name = event.process.as_ref().map(|process| process.name.as_str());
        let process_path = event
            .process
            .as_ref()
            .and_then(|process| process.path.as_deref());

        self.conn.execute(
            r#"
            INSERT OR IGNORE INTO events (
                id,
                timestamp,
                kind,
                severity,
                source,
                process_pid,
                process_name,
                process_path,
                title,
                summary,
                target,
                metadata_json,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
            params![
                event.id.as_str(),
                event.timestamp.as_str(),
                event.kind.as_str(),
                event.severity.as_str(),
                event.source.as_str(),
                process_pid,
                process_name,
                process_path,
                event.title.as_str(),
                event.summary.as_str(),
                event.target.as_deref(),
                metadata_json,
                now_timestamp().as_str(),
            ],
        )?;

        Ok(())
    }

    pub fn list_events(&self, query: EventQuery) -> DbResult<Vec<AccessEvent>> {
        let kind = query.kind.map(|kind| kind.as_str().to_string());
        let severity = query.severity.map(|severity| severity.as_str().to_string());
        let text = query
            .text
            .as_ref()
            .filter(|value| !value.trim().is_empty())
            .map(|value| format!("%{}%", value.trim()));
        let limit = query.limit.unwrap_or(100).clamp(1, 500) as i64;

        let mut statement = self.conn.prepare(
            r#"
            SELECT
                id,
                timestamp,
                kind,
                severity,
                source,
                process_pid,
                process_name,
                process_path,
                title,
                summary,
                target,
                metadata_json
            FROM events
            WHERE (?1 IS NULL OR kind = ?1)
              AND (?2 IS NULL OR severity = ?2)
              AND (
                ?3 IS NULL
                OR lower(title) LIKE lower(?3)
                OR lower(summary) LIKE lower(?3)
                OR lower(COALESCE(process_name, '')) LIKE lower(?3)
                OR lower(COALESCE(target, '')) LIKE lower(?3)
              )
            ORDER BY timestamp DESC, created_at DESC
            LIMIT ?4
            "#,
        )?;

        let rows = statement.query_map(params![kind, severity, text, limit], row_to_event)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    pub fn get_event(&self, id: &EventId) -> DbResult<Option<AccessEvent>> {
        self.conn
            .query_row(
                r#"
                SELECT
                    id,
                    timestamp,
                    kind,
                    severity,
                    source,
                    process_pid,
                    process_name,
                    process_path,
                    title,
                    summary,
                    target,
                    metadata_json
                FROM events
                WHERE id = ?1
                "#,
                params![id.as_str()],
                row_to_event,
            )
            .optional()
            .map_err(DbError::from)
    }

    pub fn seed_mock_events(&self) -> DbResult<usize> {
        let events = mock_demo_events();
        let count = events.len();
        for event in events {
            self.insert_event(&event)?;
        }
        Ok(count)
    }

    pub fn count_events(&self) -> DbResult<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
        Ok(count as u64)
    }

    pub fn dashboard_summary(&self) -> DbResult<DashboardSummary> {
        Ok(DashboardSummary {
            total_events: self.count_where("1 = 1")?,
            warnings: self.count_where("severity = 'warning'")?,
            critical: self.count_where("severity = 'critical'")?,
            process_events: self.count_where("kind = 'process'")?,
            network_events: self.count_where("kind = 'network'")?,
            file_events: self.count_where("kind = 'file'")?,
            startup_events: self.count_where("kind = 'startup'")?,
            device_access_events: self.count_where("kind = 'device_access'")?,
            locked_file_events: self.count_where("kind = 'locked_file'")?,
            security_events: self.count_where("kind = 'security'")?,
            latest_event_timestamp: self.latest_event_timestamp()?,
            demo_data_only: self.count_where("source <> 'mock'")? == 0,
        })
    }

    pub fn dev_clear_events(&self) -> DbResult<usize> {
        let changed = self.conn.execute("DELETE FROM events", [])?;
        Ok(changed)
    }

    fn count_where(&self, condition: &str) -> DbResult<u64> {
        let sql = format!("SELECT COUNT(*) FROM events WHERE {condition}");
        let count: i64 = self.conn.query_row(&sql, [], |row| row.get(0))?;
        Ok(count as u64)
    }

    fn latest_event_timestamp(&self) -> DbResult<Option<EventTimestamp>> {
        let timestamp: Option<String> = self
            .conn
            .query_row(
                "SELECT timestamp FROM events ORDER BY timestamp DESC, created_at DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(timestamp.map(EventTimestamp::new))
    }
}

fn row_to_event(row: &rusqlite::Row<'_>) -> rusqlite::Result<AccessEvent> {
    let id: String = row.get(0)?;
    let timestamp: String = row.get(1)?;
    let kind: String = row.get(2)?;
    let severity: String = row.get(3)?;
    let source: String = row.get(4)?;
    let process_pid: Option<i64> = row.get(5)?;
    let process_name: Option<String> = row.get(6)?;
    let process_path: Option<String> = row.get(7)?;
    let title: String = row.get(8)?;
    let summary: String = row.get(9)?;
    let target: Option<String> = row.get(10)?;
    let metadata_json: Option<String> = row.get(11)?;

    let process = process_name.map(|name| ProcessRef {
        pid: process_pid.and_then(|pid| u32::try_from(pid).ok()),
        name,
        path: process_path,
    });

    let metadata_json = metadata_json.and_then(|value| serde_json::from_str::<Value>(&value).ok());

    Ok(AccessEvent {
        id: EventId::new(id),
        timestamp: EventTimestamp::new(timestamp),
        kind: EventKind::from_str(&kind)
            .map_err(|error| rusqlite::Error::FromSqlConversionFailure(2, Type::Text, Box::new(error)))?,
        severity: EventSeverity::from_str(&severity)
            .map_err(|error| rusqlite::Error::FromSqlConversionFailure(3, Type::Text, Box::new(error)))?,
        source: EventSource::from_str(&source)
            .map_err(|error| rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(error)))?,
        process,
        title,
        summary,
        target,
        metadata_json,
        confidence: None,
    })
}

fn now_timestamp() -> EventTimestamp {
    EventTimestamp::now_utc_best_effort()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsentinel_events::{mock_network_event, mock_process_event};

    fn temp_db_path(test_name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("jsentinel-{test_name}-{nanos}.sqlite3"))
    }

    #[test]
    fn init_db_creates_schema() {
        let database = init_db(temp_db_path("init")).expect("database should initialize");
        assert_eq!(database.count_events().expect("count should work"), 0);
    }

    #[test]
    fn insert_and_list_event() {
        let database = init_db(temp_db_path("insert-list")).expect("database should initialize");
        database
            .insert_event(&mock_process_event())
            .expect("event insert should work");

        let events = database
            .list_events(EventQuery::default())
            .expect("event list should work");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, EventKind::Process);
        assert_eq!(events[0].source, EventSource::Mock);
    }

    #[test]
    fn query_filters_by_kind_and_severity() {
        let database = init_db(temp_db_path("filters")).expect("database should initialize");
        database
            .insert_event(&mock_process_event())
            .expect("process event insert should work");
        database
            .insert_event(&mock_network_event())
            .expect("network event insert should work");

        let warnings = database
            .list_events(EventQuery {
                kind: Some(EventKind::Network),
                severity: Some(EventSeverity::Warning),
                text: None,
                limit: Some(10),
            })
            .expect("filtered query should work");

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].kind, EventKind::Network);
    }

    #[test]
    fn query_searches_text_fields() {
        let database = init_db(temp_db_path("text-search")).expect("database should initialize");
        database
            .insert_event(&mock_process_event())
            .expect("process event insert should work");
        database
            .insert_event(&mock_network_event())
            .expect("network event insert should work");

        let events = database
            .list_events(EventQuery {
                kind: None,
                severity: None,
                text: Some("example.invalid".to_string()),
                limit: Some(10),
            })
            .expect("text query should work");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, EventKind::Network);
    }

    #[test]
    fn dashboard_summary_counts_events() {
        let database = init_db(temp_db_path("summary")).expect("database should initialize");
        database
            .seed_mock_events()
            .expect("mock seed should insert events");

        let summary = database
            .dashboard_summary()
            .expect("summary should be available");

        assert_eq!(summary.total_events, 7);
        assert_eq!(summary.warnings, 3);
        assert_eq!(summary.critical, 1);
        assert_eq!(summary.process_events, 1);
        assert!(summary.demo_data_only);
    }
}
