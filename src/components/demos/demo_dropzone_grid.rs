use dioxus::prelude::*;
use icons::Upload;

use crate::components::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneFileGrid, DropzoneHint, DropzoneIcon, DropzoneLabel,
};

#[component]
pub fn DemoDropzoneGrid() -> Element {
    rsx! {
        div { class: "max-w-[700px] mx-auto w-full",
            Dropzone {
                div { class: "space-y-4",
                    div { class: "space-y-1",
                        h2 { class: "text-base font-semibold", "Upload files" }
                        p { class: "text-sm text-muted-foreground",
                            "Drag and drop your files here or click to browse."
                        }
                    }
                    DropzoneArea {
                        DropzoneIcon { Upload { class: "size-7" } }
                        DropzoneLabel { "Drag 'n' drop files here, or click to select files" }
                        DropzoneHint { "You can upload 8 files (up to 8 MB each)" }
                    }
                    DropzoneFileGrid {}
                }
            }
        }
    }
}
