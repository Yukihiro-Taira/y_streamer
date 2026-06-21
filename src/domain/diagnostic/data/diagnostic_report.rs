use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiagnosticStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub label: String,
    pub status: DiagnosticStatus,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticReport {
    pub checks: Vec<DiagnosticCheck>,
}

impl DiagnosticReport {
    pub fn pass_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Pass))
            .count()
    }

    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Warn))
            .count()
    }

    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, DiagnosticStatus::Fail))
            .count()
    }
}
