use std::sync::{Arc, Mutex};

use clap::{Parser, ValueEnum};
use multi_progressbar::{
    bar::block::BlockProgressBar, bar::classic::ClassicProgressBar, MultiProgressBar, ProgressBar,
    TaskProgress,
};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, default_value = "1")]
    num_tasks: usize,
    #[clap(long, default_value = "classic")]
    bar_type: BarType,
}

#[derive(Debug, Clone, ValueEnum)]
enum BarType {
    Classic,
    Block,
}

struct Task {
    id: usize,
    progress: u64,
    total: u64,
}

impl Task {
    fn new(id: usize, total: u64) -> Self {
        Task {
            id,
            progress: 0,
            total,
        }
    }
    fn inc(&mut self) {
        if self.progress >= self.total {
            return;
        }
        self.progress += 1;
    }
}

impl TaskProgress for Task {
    fn progress(&self) -> (u64, u64) {
        (self.progress, self.total)
    }
    fn before(&self) -> Option<String> {
        Some(format!("task {}", self.id))
    }
}

fn run_demo<P>(cli: &Cli, mp: MultiProgressBar<P>)
where
    P: ProgressBar<Task = Task>,
{
    let tasks = (0..cli.num_tasks)
        .map(|i| Task::new(i, 20))
        .collect::<Vec<_>>();

    let tasks = Arc::new(Mutex::new(tasks));

    let tasks_clone = tasks.clone();

    let drive_task = move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut tasks = tasks_clone.lock().unwrap();
        for task in tasks.iter_mut() {
            task.inc();
        }
    };

    std::thread::spawn(drive_task);

    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let tasks = tasks.lock().unwrap();

        {
            mp.log(format!("log {}", i).as_str(), tasks.len()).unwrap();
            mp.draw(tasks.as_slice()).unwrap();
        }

        if tasks.is_empty() {
            break;
        }
        i += 1;
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.bar_type {
        BarType::Classic => {
            let mp = MultiProgressBar::new(ClassicProgressBar::new());
            run_demo(&cli, mp);
        }
        BarType::Block => {
            let mp = MultiProgressBar::new(BlockProgressBar::new());
            run_demo(&cli, mp);
        }
    };
}
