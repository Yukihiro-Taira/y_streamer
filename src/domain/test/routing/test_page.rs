use dioxus::prelude::*;

use crate::components::demos::demo_dropzone::DemoDropzone;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "max-w-[800px] mx-auto w-full px-6 py-8 space-y-4", DemoDropzone {} }
    }
}
