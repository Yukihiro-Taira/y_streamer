use dioxus::prelude::*;
use icons::Upload;
use tw_merge::tw_merge;

#[component]
pub fn FileDropzone(
    is_dragging: bool,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    if !is_dragging {
        return rsx! {};
    }

    let overlay_class = tw_merge!(
        "fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-background/70 pointer-events-none",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "{overlay_class}",
            div {
                class: "flex flex-col items-center gap-5 rounded-2xl border-2 border-dashed border-primary bg-background/80 px-20 py-14 shadow-2xl text-primary select-none",
                Upload { class: "size-14 animate-bounce" }
                p { class: "text-xl font-semibold tracking-tight", "Drop files here" }
                p { class: "text-sm text-muted-foreground font-normal", "Release to upload" }
            }
        }
    }
}
