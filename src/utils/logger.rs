//! Operation logger for undo functionality

use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single file operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub from: PathBuf,
    pub to: PathBuf,
    pub operation_type: OperationType,
}

/// Type of operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Move,
    Delete,
}

/// A batch of operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationBatch {
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub operations: Vec<FileOperation>,
}

/// Operation history
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub batches: Vec<OperationBatch>,
}

impl History {
    /// Get the history file path
    fn history_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not find home directory")?;
        let neat_dir = home.join(".neat");
        fs::create_dir_all(&neat_dir)?;
        Ok(neat_dir.join("history.json"))
    }

    /// Load history from file
    pub fn load() -> Result<Self> {
        let path = Self::history_path()?;

        if !path.exists() {
            return Ok(History::default());
        }

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Ok(History::default()),
        };

        let reader = BufReader::new(file);

        // If the file is corrupted, just return empty history
        // This prevents tests and operations from failing due to old/corrupted data
        match serde_json::from_reader(reader) {
            Ok(history) => Ok(history),
            Err(e) => {
                eprintln!("Warning: History file corrupted ({}), starting fresh.", e);
                // Delete the corrupted file
                let _ = fs::remove_file(&path);
                Ok(History::default())
            }
        }
    }

    /// Save history to file
    pub fn save(&self) -> Result<()> {
        let path = Self::history_path()?;
        let file = File::create(&path).context("Failed to create history file")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).context("Failed to write history file")?;
        Ok(())
    }

    /// Add a new batch of operations
    pub fn add_batch(&mut self, command: String, operations: Vec<FileOperation>) {
        let batch = OperationBatch {
            timestamp: Utc::now(),
            command,
            operations,
        };
        self.batches.push(batch);

        // Keep only the last 50 batches
        if self.batches.len() > 50 {
            self.batches.remove(0);
        }
    }

    /// Get the last batch for undo
    pub fn pop_last(&mut self) -> Option<OperationBatch> {
        self.batches.pop()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }
}

/// Logger for tracking operations
pub struct Logger {
    operations: Vec<FileOperation>,
    command: String,
}

impl Logger {
    /// Create a new logger for a command
    pub fn new(command: &str) -> Self {
        Logger {
            operations: Vec::new(),
            command: command.to_string(),
        }
    }

    /// Log a move operation
    pub fn log_move(&mut self, from: PathBuf, to: PathBuf) {
        self.operations.push(FileOperation {
            from,
            to,
            operation_type: OperationType::Move,
        });
    }

    /// Log a delete operation
    pub fn log_delete(&mut self, path: PathBuf) {
        self.operations.push(FileOperation {
            from: path,
            to: PathBuf::new(),
            operation_type: OperationType::Delete,
        });
    }

    /// Save logged operations to history
    pub fn save(self) -> Result<()> {
        if self.operations.is_empty() {
            return Ok(());
        }

        let mut history = History::load()?;
        history.add_batch(self.command, self.operations);
        history.save()?;
        Ok(())
    }

    /// Get the count of logged operations
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.operations.len()
    }
}
