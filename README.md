# multi-progress

multi-progress is a library to show multiple progress bars along with log outputs in terminal.

## Usage

1. Implement `TaskProgress` trait for your task.
2. Create a `MultiProgressBar` with a `ProgressBar` implementation (provided in the bar module).
3. Call `MultiProgressBar::draw` to draw progress bars when needed.

```rust
use multi_progressbar::{
    MultiProgressBar, TaskProgress,
    bar::classic::ClassicProgressBar
};

struct Task {
    name: String,
    progress: u64,
    total: u64,
}

impl TaskProgress for Task {
    fn progress(&self) -> (u64, u64) {
        (self.progress, self.total)
    }
    fn after(&self) -> Option<String> {
        Some(format!("{}/{} completed", self.progress, self.total))
    }
    fn before(&self) -> Option<String> {
        Some(self.name.clone())
    }
}

let mp = MultiProgressBar::new(ClassicProgressBar::new());
let task1 = Task {
   name: "task1".to_string(),
   progress: 0,
   total: 100,
};
let task2 = Task {
    name: "task2".to_string(),
    progress: 33,
    total: 100,
};
let tasks = vec![task1, task2];
mp.draw(&tasks).unwrap();
```