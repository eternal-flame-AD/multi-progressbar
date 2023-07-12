//! # multi-progress
//!
//! multi-progress is a library to show multiple progress bars along with log outputs in terminal.
//!
//! ## Usage
//!
//! 1. Implement [TaskProgress] trait for your task.
//! 2. Call [MultiProgressBar::new] with a [ProgressBar] implementation (provided in the [bar] module).
//! 3. Call [MultiProgressBar::draw] to draw progress bars when needed.
//!
//! ```rust
//! use multi_progressbar::{
//!     MultiProgressBar, TaskProgress,
//!     bar::classic::ClassicProgressBar
//! };
//!
//! struct Task {
//!     name: String,
//!     progress: u64,
//!     total: u64,
//! }
//!
//! impl TaskProgress for Task {
//!     fn progress(&self) -> (u64, u64) {
//!         (self.progress, self.total)
//!     }
//!     fn after(&self) -> Option<String> {
//!         Some(format!("{}/{} completed", self.progress, self.total))
//!     }
//!     fn before(&self) -> Option<String> {
//!         Some(self.name.clone())
//!     }
//! }
//!
//! let mp = MultiProgressBar::new(ClassicProgressBar::new());
//! let task1 = Task {
//!    name: "task1".to_string(),
//!    progress: 0,
//!    total: 100,
//! };
//! let task2 = Task {
//!     name: "task2".to_string(),
//!     progress: 33,
//!     total: 100,
//! };
//! let tasks = vec![task1, task2];
//! mp.draw(&tasks).unwrap();
//!
//!
//! ```

#![warn(missing_docs)]

use crossterm::{cursor, queue, terminal};
use std::io::Write;

/// bar module contains premade progress bar styles.
pub mod bar;

/// Task is abstraction for one single running task.
pub trait TaskProgress {
    /// returns the current progress and total progress.
    fn progress(&self) -> (u64, u64);
    /// returns message to show before progress bar
    fn before(&self) -> Option<String> {
        None
    }
    /// returns message to show after progress bar
    fn after(&self) -> Option<String> {
        None
    }
}

/// ProgressBar is an abstraction for the appearance of a progress bar.
pub trait ProgressBar {
    /// Progress is provided by TaskProgress.
    type Task: TaskProgress;
    /// formats a line of progress bar to show in terminal.
    fn format_line(&self, progress: &Self::Task, width: usize) -> String;
}

/// MultiProgress is the main struct of this library.
/// It handles drawing progress bars and log outputs.
pub struct MultiProgressBar<P: ProgressBar> {
    progress_bar: P,
}

impl<P: ProgressBar> MultiProgressBar<P> {
    /// creates a new MultiProgress with given ProgressBar style.
    pub fn new(progress_bar: P) -> Self {
        MultiProgressBar { progress_bar }
    }

    /// logs a message above progress bars.
    pub fn log(&self, msg: &str, ntasks: usize) -> std::io::Result<()> {
        let (width, height) = crossterm::terminal::size().unwrap();
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            cursor::MoveToRow(height - ntasks as u16 - 1),
            cursor::MoveToColumn(0),
            terminal::ScrollUp(1),
        )?;

        write!(stdout, "{:width$}", msg, width = width as usize)
    }

    /// draws the progress bars.
    pub fn draw(&self, tasks: &[P::Task]) -> std::io::Result<()> {
        let (width, height) = crossterm::terminal::size().unwrap();
        let mut stdout = std::io::stdout();
        queue!(
            stdout,
            terminal::BeginSynchronizedUpdate,
            cursor::MoveToColumn(0),
            cursor::MoveToRow(height - tasks.len() as u16 - 1),
        )?;

        for task in tasks {
            let line = self.progress_bar.format_line(task, width as usize);
            queue!(stdout, cursor::MoveToColumn(0), cursor::MoveDown(1))?;
            write!(stdout, "{}", line)?;
        }

        queue!(
            stdout,
            terminal::EndSynchronizedUpdate,
            cursor::MoveToColumn(0),
        )?;

        stdout.flush()?;

        Ok(())
    }
}
