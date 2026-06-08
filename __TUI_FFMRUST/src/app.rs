use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crate::job::{Job, demo_jobs};

const JOB_DURATION: Duration = Duration::from_secs(10);
const CELEBRATION_DURATION_TICKS: u64 = 48;
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Failed,
}

pub struct App {
    jobs: Vec<Job>,
    dropped_documents: Vec<PathBuf>,
    current_job: usize,
    job_started_at: Instant,
    elapsed_before_pause: Duration,
    spinner_index: usize,
    is_complete: bool,
    is_paused: bool,
    frame_tick: u64,
    celebration_until_tick: Option<u64>,
}

impl App {
    pub fn new() -> Self {
        Self {
            jobs: demo_jobs(),
            dropped_documents: Vec::new(),
            current_job: 0,
            job_started_at: Instant::now(),
            elapsed_before_pause: Duration::ZERO,
            spinner_index: 0,
            is_complete: false,
            is_paused: true,
            frame_tick: 0,
            celebration_until_tick: None,
        }
    }

    pub fn update(&mut self) {
        self.frame_tick = self.frame_tick.wrapping_add(1);

        if let Some(until_tick) = self.celebration_until_tick {
            if self.frame_tick >= until_tick {
                self.celebration_until_tick = None;
            }
        }

        if self.is_complete {
            return;
        }

        if self.is_paused {
            return;
        }

        self.spinner_index = (self.spinner_index + 1) % SPINNER_FRAMES.len();

        if self.job_elapsed() >= JOB_DURATION {
            if self.current_job + 1 < self.jobs.len() {
                self.current_job += 1;
                self.job_started_at = Instant::now();
                self.elapsed_before_pause = Duration::ZERO;
            } else {
                self.is_complete = true;
            }
        }
    }

    pub fn current_job(&self) -> &Job {
        &self.jobs[self.current_job]
    }

    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }

    pub fn dropped_documents(&self) -> &[PathBuf] {
        &self.dropped_documents
    }

    pub fn dropped_document_count(&self) -> usize {
        self.dropped_documents.len()
    }

    pub fn add_dropped_document(&mut self, path: PathBuf) {
        if !self.dropped_documents.iter().any(|existing| existing == &path) {
            self.dropped_documents.push(path);
        }
    }

    pub fn document_label(path: &Path) -> String {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| path.display().to_string())
    }

    pub fn current_job_index(&self) -> usize {
        self.current_job
    }

    pub fn job_status(&self, index: usize) -> JobStatus {
        if self.jobs[index].failure_reason.is_some() && index > self.current_job {
            return JobStatus::Failed;
        }

        if index < self.current_job {
            JobStatus::Done
        } else if index == self.current_job {
            if self.is_complete {
                JobStatus::Done
            } else {
                JobStatus::Running
            }
        } else {
            JobStatus::Queued
        }
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn current_job_status(&self) -> JobStatus {
        self.job_status(self.current_job)
    }

    pub fn frame_tick(&self) -> u64 {
        self.frame_tick
    }

    pub fn trigger_celebration(&mut self) {
        self.celebration_until_tick = Some(self.frame_tick + CELEBRATION_DURATION_TICKS);
    }

    pub fn should_celebrate(&self) -> bool {
        self.celebration_until_tick.is_some()
    }

    pub fn progress_ratio(&self) -> f64 {
        if self.is_complete {
            return 1.0;
        }
        (self.job_elapsed().as_secs_f64() / JOB_DURATION.as_secs_f64()).clamp(0.0, 1.0)
    }

    pub fn progress_percent(&self) -> u16 {
        (self.progress_ratio() * 100.0).round() as u16
    }

    pub fn spinner_frame(&self) -> &'static str {
        if self.is_complete {
            return "✓";
        }
        if self.is_paused {
            return "⏸";
        }
        SPINNER_FRAMES[self.spinner_index]
    }

    pub fn pending_jobs(&self) -> usize {
        self.jobs
            .iter()
            .enumerate()
            .filter(|(index, _)| self.job_status(*index) == JobStatus::Queued)
            .count()
    }

    pub fn completed_jobs(&self) -> usize {
        self.jobs
            .iter()
            .enumerate()
            .filter(|(index, _)| self.job_status(*index) == JobStatus::Done)
            .count()
    }

    pub fn total_jobs(&self) -> usize {
        self.jobs.len()
    }

    pub fn move_job(&mut self, from_index: usize, to_index: usize) {
        if from_index >= self.jobs.len() || to_index >= self.jobs.len() || from_index == to_index {
            return;
        }

        let job = self.jobs.remove(from_index);
        self.jobs.insert(to_index, job);

        if self.current_job == from_index {
            self.current_job = to_index;
        } else if from_index < self.current_job && to_index >= self.current_job {
            self.current_job = self.current_job.saturating_sub(1);
        } else if from_index > self.current_job && to_index <= self.current_job {
            self.current_job += 1;
        }
    }

    pub fn elapsed_label(&self) -> String {
        format_duration(self.job_elapsed())
    }

    pub fn speed_label(&self) -> &'static str {
        if self.is_complete {
            return "done";
        }
        if self.is_paused {
            return "paused";
        }
        match self.progress_ratio() {
            p if p < 0.25 => "1.2x",
            p if p < 0.5 => "1.6x",
            p if p < 0.75 => "1.8x",
            _ => "2.1x",
        }
    }

    pub fn output_size_mb(&self) -> u64 {
        let target = self.current_job().target_output_mb as f64;
        if self.is_complete {
            target.round() as u64
        } else {
            (target * self.progress_ratio()).round() as u64
        }
    }

    pub fn saved_mb(&self) -> u64 {
        self.current_job()
            .input_size_mb
            .saturating_sub(self.current_job().target_output_mb)
    }

    pub fn reduction_percent(&self) -> u16 {
        let job = self.current_job();
        if job.input_size_mb == 0 {
            return 0;
        }

        let saved = job.input_size_mb.saturating_sub(job.target_output_mb) as f64;
        ((saved / job.input_size_mb as f64) * 100.0).round() as u16
    }

    pub fn toggle_pause(&mut self) {
        if self.is_complete {
            return;
        }

        if self.is_paused {
            self.is_paused = false;
            self.job_started_at = Instant::now();
        } else {
            self.elapsed_before_pause = self.job_elapsed();
            self.is_paused = true;
        }
    }

    fn job_elapsed(&self) -> Duration {
        if self.is_paused {
            self.elapsed_before_pause
        } else {
            self.elapsed_before_pause + self.job_started_at.elapsed()
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("00:{minutes:02}:{seconds:02}")
}
