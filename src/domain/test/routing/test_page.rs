use dioxus::prelude::*;

use crate::components::demos::demo_dropzone::DemoDropzone;
use crate::components::demos::demo_dropzone_grid::DemoDropzoneGrid;
use crate::components::demos::demo_video_inspector::DemoVideoInspector;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "max-w-[800px] mx-auto w-full px-6 py-8 space-y-12",
            DemoVideoInspector {}
            DemoDropzone {}
            DemoDropzoneGrid {}
        }
    }
}
