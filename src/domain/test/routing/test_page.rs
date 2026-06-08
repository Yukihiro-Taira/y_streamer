use dioxus::prelude::*;

use crate::components::demos::demo_dropzone::DemoDropzone;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "space-y-6 pt-4",
            h1 { class: "text-xl font-semibold", "File Drop Test" }
            DemoDropzone {}
        }
    }
}
