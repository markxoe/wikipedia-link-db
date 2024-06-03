#![allow(dead_code)] // todo: remove this when tests are implemented
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

fn progressbar_template(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
       ProgressStyle::with_template(
           "{prefix:.bold.dim} [{elapsed_precise:.yellow}] [{bar:40.green/yellow}] {pos:>9}/{len:9} {msg}",
       )
       .unwrap()
       .progress_chars("##-"),
   );

    pb
}

pub fn spinner(with_prefix: bool) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("-\\|/#")
            .template(if with_prefix {
                "{prefix:>5.bold.dim} [{elapsed_precise:.yellow}] {spinner:.green} {msg}"
            } else {
                "[{elapsed_precise:.yellow}] {spinner:.green} {msg}"
            })
            .unwrap(),
    );

    pb
}

pub struct ProgressReporter {
    progress: Option<ProgressBar>,
    finish_message: Option<&'static str>,
}

impl ProgressReporter {
    pub fn new_progress(
        step: u8,
        steps: u8,
        message: String,
        finish_message: &'static str,
        len: u64,
    ) -> Self {
        let progress = progressbar_template(len);
        progress.set_prefix(format!("[{}/{}]", step, steps));
        progress.set_message(message.clone());

        Self {
            progress: Some(progress),
            finish_message: Some(finish_message),
        }
    }

    pub fn new_spinner(
        message: String,
        finish_message: Option<&'static str>,
        steps: Option<(u8, u8)>,
    ) -> Self {
        let progress = spinner(if let Some(_) = steps { true } else { false });
        progress.set_message(message.clone());

        if let Some((step, steps)) = steps {
            progress.set_prefix(format!("[{}/{}]", step, steps));
        }

        Self {
            progress: Some(progress),
            finish_message,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            progress: None,
            finish_message: Some(""),
        }
    }

    pub fn inc(&self, count: u64) {
        if let Some(pb) = &self.progress {
            pb.inc(count);
        }
    }

    pub fn enable_background(&self) {
        if let Some(pb) = &self.progress {
            pb.enable_steady_tick(Duration::from_millis(100));
        }
    }

    pub fn finish(&self) {
        if let Some(pb) = &self.progress {
            if let Some(message) = self.finish_message {
                pb.finish_with_message(message);
            } else {
                pb.finish();
            }
        }
    }
}

enum ProgressType {
    Progress,
    Spinner,
    Empty,
}

pub struct ProgressBuilder {
    bar_type: ProgressType,
    len: Option<u64>,
    message: Option<String>,
    finish_message: Option<&'static str>,
    steps: Option<u8>,
    step: Option<u8>,
}

impl ProgressBuilder {
    pub fn empty() -> Self {
        Self {
            bar_type: ProgressType::Empty,
            len: None,
            message: None,
            finish_message: None,
            steps: None,
            step: None,
        }
    }
    pub fn new() -> Self {
        Self {
            bar_type: ProgressType::Progress,
            len: None,
            message: None,
            finish_message: None,
            steps: None,
            step: None,
        }
    }

    pub fn spinner() -> Self {
        Self {
            bar_type: ProgressType::Spinner,
            len: None,
            message: None,
            finish_message: None,
            steps: None,
            step: None,
        }
    }

    pub fn with_len(mut self, len: u64) -> Self {
        self.len = Some(len);
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn with_finish_message(mut self, message: &'static str) -> Self {
        self.finish_message = Some(message);
        self
    }

    pub fn with_steps(mut self, step: u8, steps: u8) -> Self {
        self.steps = Some(steps);
        self.step = Some(step);
        self
    }

    fn build_empty(self) -> ProgressReporter {
        ProgressReporter::new_empty()
    }

    fn build_progress(self) -> ProgressReporter {
        let len = self.len.unwrap() as u64;
        let message = self.message.unwrap();
        let steps = self.steps.unwrap();
        let step = self.step.unwrap();

        let finish_message = self.finish_message.unwrap();

        ProgressReporter::new_progress(step, steps, message, finish_message, len)
    }

    fn build_spinner(self) -> ProgressReporter {
        let message = self.message.unwrap();
        let finish_message = self.finish_message;

        let steps = if let Some(steps) = self.steps {
            if let Some(step) = self.step {
                Some((step, steps))
            } else {
                None
            }
        } else {
            None
        };

        ProgressReporter::new_spinner(message, finish_message, steps)
    }

    pub fn build(self) -> ProgressReporter {
        match self.bar_type {
            ProgressType::Progress => self.build_progress(),
            ProgressType::Spinner => self.build_spinner(),
            ProgressType::Empty => self.build_empty(),
        }
    }
}
