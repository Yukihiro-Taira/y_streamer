use dioxus::prelude::*;
use icons::Upload;

use crate::components::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneFileList, DropzoneHint, DropzoneIcon, DropzoneLabel,
    DropzoneOverlay,
};

#[component]
pub fn DemoDropzone() -> Element {
    rsx! {
        Dropzone {
            DropzoneOverlay {}
            div { class: "space-y-4",
                DropzoneArea {
                    DropzoneIcon {
                        Upload { class: "size-8" }
                    }
                    DropzoneLabel { "Drag 'n' drop files here, or click to select files" }
                    DropzoneHint { "You can upload 8 files (up to 8 MB each)" }
                }
                DropzoneFileList {}
            }
        }
    }
}
