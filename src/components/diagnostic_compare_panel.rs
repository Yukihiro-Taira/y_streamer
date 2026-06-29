use dioxus::prelude::*;

use crate::domain::diagnostic::data::diagnostic_comparison::{
    CompareFieldRow, build_compare_rows,
};
use crate::domain::diagnostic::data::diagnostic_report::DiagnosticReport;
use crate::domain::media_read::data::media_probe_report::MediaProbeReport;

#[component]
pub fn ComparisonPanel(
    left_report: MediaProbeReport,
    left_diag: DiagnosticReport,
    right_report: MediaProbeReport,
    right_diag: DiagnosticReport,
) -> Element {
    let rows = build_compare_rows(&left_report, &right_report);
    let diff_rows = rows.iter().filter(|row| !row.same).cloned().collect::<Vec<_>>();
    let diff_count = diff_rows.len();

    rsx! {
        div { class: "space-y-3",
            div { class: "space-y-1",
                h2 { class: "text-base font-semibold", "Comparison" }
                p { class: "text-sm text-muted-foreground",
                    "Simple side-by-side diff between File A and File B, with the most useful metadata for export audits."
                }
            }
            div { class: "flex flex-wrap gap-2",
                span {
                    class: if diff_count == 0 {
                        "inline-flex items-center rounded-full bg-green-100 px-2.5 py-1 text-xs font-semibold text-green-800 dark:bg-green-900/30 dark:text-green-400"
                    } else {
                        "inline-flex items-center rounded-full bg-yellow-100 px-2.5 py-1 text-xs font-semibold text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400"
                    },
                    if diff_count == 0 {
                        "All compared fields match"
                    } else {
                        "{diff_count} field(s) differ"
                    }
                }
            }
            div { class: "grid gap-3 md:grid-cols-2",
                CompactCompareCard { slot: "File A".to_string(), report: left_report.clone(), diag: left_diag }
                CompactCompareCard { slot: "File B".to_string(), report: right_report.clone(), diag: right_diag }
            }
            if !diff_rows.is_empty() {
                div { class: "rounded-xl border border-border bg-card px-4 py-3 space-y-2",
                    p { class: "text-sm font-semibold", "Notable differences" }
                    div { class: "flex flex-wrap gap-2",
                        for row in diff_rows.iter() {
                            span { class: "inline-flex items-center rounded-full border border-border bg-muted/40 px-2.5 py-1 text-[11px] text-foreground",
                                "{row.label}"
                            }
                        }
                    }
                }
            }
            div { class: "rounded-xl border border-border overflow-hidden",
                div { class: "grid grid-cols-[minmax(120px,1.1fr)_minmax(0,1fr)_minmax(0,1fr)_88px] gap-0 bg-muted/40 px-4 py-3 text-xs font-semibold uppercase tracking-wide text-muted-foreground",
                    span { "Field" }
                    span { "File A" }
                    span { "File B" }
                    span { "Status" }
                }
                div { class: "divide-y divide-border",
                    for row in rows {
                        CompareRow {
                            label: row.label,
                            left_value: row.left_value,
                            right_value: row.right_value,
                            same: row.same,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CompactCompareCard(slot: String, report: MediaProbeReport, diag: DiagnosticReport) -> Element {
    let verdict_class = if diag.fail_count() > 0 {
        "text-red-700 dark:text-red-400"
    } else if diag.warn_count() > 0 {
        "text-yellow-700 dark:text-yellow-400"
    } else {
        "text-green-700 dark:text-green-400"
    };
    let verdict = if diag.fail_count() > 0 {
        format!("{} fail", diag.fail_count())
    } else if diag.warn_count() > 0 {
        format!("{} warn", diag.warn_count())
    } else {
        "all pass".to_string()
    };

    rsx! {
        div { class: "rounded-xl border border-border bg-card px-4 py-3 space-y-1",
            div { class: "flex items-center justify-between gap-2",
                span { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground", "{slot}" }
                span { class: "text-xs font-semibold {verdict_class}", "{verdict}" }
            }
            p { class: "text-sm font-medium truncate", "{report.file_name}" }
            p { class: "text-xs text-muted-foreground",
                "{report.format_name}"
                if !report.duration.is_empty() {
                    " · {report.duration}"
                }
            }
        }
    }
}

#[component]
fn CompareRow(label: String, left_value: String, right_value: String, same: bool) -> Element {
    let badge_class = if same {
        "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400"
    } else {
        "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400"
    };
    let badge_label = if same { "same" } else { "diff" };

    rsx! {
        div { class: "grid grid-cols-[minmax(120px,1.1fr)_minmax(0,1fr)_minmax(0,1fr)_88px] gap-0 px-4 py-3 text-sm",
            span { class: "font-medium text-foreground", "{label}" }
            span { class: "font-mono text-xs text-muted-foreground break-all pr-4", "{left_value}" }
            span { class: "font-mono text-xs text-muted-foreground break-all pr-4", "{right_value}" }
            span {
                class: "inline-flex h-fit items-center justify-center rounded-full px-2 py-0.5 text-[11px] font-semibold {badge_class}",
                "{badge_label}"
            }
        }
    }
}

#[allow(dead_code)]
fn _assert_compare_row_type(_: &CompareFieldRow) {}
