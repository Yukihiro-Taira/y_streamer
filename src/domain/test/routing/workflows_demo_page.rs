use dioxus::prelude::*;

use crate::components::demos::workflow01::Workflow01;
use crate::components::demos::workflow02::Workflow02;
use crate::components::demos::workflow03::Workflow03;

#[component]
pub fn WorkflowsDemoPage() -> Element {
    rsx! {
        div { class: "max-w-[900px] mx-auto w-full px-6 py-8 space-y-12",
            section { class: "space-y-3",
                h2 { class: "text-sm font-semibold text-muted-foreground uppercase tracking-widest", "Workflow 01 — Basic" }
                Workflow01 {}
            }
            section { class: "space-y-3",
                h2 { class: "text-sm font-semibold text-muted-foreground uppercase tracking-widest", "Workflow 02 — Copy / Paste" }
                Workflow02 {}
            }
            section { class: "space-y-3",
                h2 { class: "text-sm font-semibold text-muted-foreground uppercase tracking-widest", "Workflow 03 — Keyboard Nudge" }
                Workflow03 {}
            }
        }
    }
}
