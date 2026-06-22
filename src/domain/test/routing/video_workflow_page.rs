use dioxus::prelude::*;
use icons::Upload;

use crate::components::hooks::use_workflow::{
    WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow,
};
use crate::components::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneCtx, DropzoneHint, DropzoneIcon, DropzoneLabel,
};
use crate::components::workflow::{
    WfNode, WfNodeContent, WfNodeHeader, WfNodeTitle, WorkflowCanvas, WorkflowControls,
    WorkflowDefaultNode, WorkflowNodeWrapper,
};

fn pipeline_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "input".to_string(),
            initial_x: 32.0,
            initial_y: 170.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Drop a video".to_string(),
            description: "waiting for file…".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "ffmpeg".to_string(),
            initial_x: 280.0,
            initial_y: 170.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "FFmpeg".to_string(),
            description: "process video".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "thumbnails".to_string(),
            initial_x: 530.0,
            initial_y: 80.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Thumbnails".to_string(),
            description: "3 frames extracted".to_string(),
            kind: WorkflowNodeKind::Output,
        },
        WorkflowNode {
            id: "subtitles".to_string(),
            initial_x: 530.0,
            initial_y: 260.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Subtitles".to_string(),
            description: ".srt / .vtt streams".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn pipeline_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge {
            from: "input".to_string(),
            to: "ffmpeg".to_string(),
            ..Default::default()
        },
        WorkflowEdge {
            from: "ffmpeg".to_string(),
            to: "thumbnails".to_string(),
            ..Default::default()
        },
        WorkflowEdge {
            from: "ffmpeg".to_string(),
            to: "subtitles".to_string(),
            ..Default::default()
        },
    ]
}

/// Custom node for "thumbnails" — shows a video preview from DropzoneCtx
#[component]
fn ThumbnailsNode(node: WorkflowNode) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let files = ctx.files.read();
    let preview = files.first().and_then(|f| f.preview_url.clone());

    rsx! {
        WfNode {
            target: false,
            source: false,
            WfNodeHeader {
                span { class: format!("size-2 rounded-full shrink-0 {}", node.kind.dot_color()) }
                WfNodeTitle { class: node.kind.text_color(), "{node.label}" }
                span {
                    class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                    "{node.kind.label()}"
                }
            }
            WfNodeContent {
                if let Some(url) = preview {
                    video {
                        src: "{url}",
                        class: "w-full rounded object-cover bg-muted",
                        style: "height: 80px;",
                        preload: "metadata",
                        muted: true,
                    }
                } else {
                    p { class: "text-[10px] text-muted-foreground font-mono", "waiting for file…" }
                }
            }
        }
    }
}

#[component]
fn VideoWorkflowCanvas() -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let mut state = use_workflow(pipeline_nodes(), pipeline_edges());

    use_effect(move || {
        state.set_locked(true);
    });

    use_effect(move || {
        let files = ctx.files.read();
        if let Some(file) = files.first() {
            let name = file.name.clone();
            let size = file.size_display();
            let mut nodes = state.nodes.write();
            if let Some(n) = nodes.iter_mut().find(|n| n.id == "input") {
                n.label = name;
                n.description = size;
            }
        }
    });

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! { WorkflowControls { state } },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    if node.id == "thumbnails" {
                        ThumbnailsNode { node }
                    } else {
                        WorkflowDefaultNode { node }
                    }
                }
            }
        }
    }
}

#[component]
pub fn VideoWorkflowPage() -> Element {
    rsx! {
        div { class: "max-w-[900px] mx-auto w-full px-6 py-8 space-y-6",
            Dropzone {
                div { class: "space-y-6",
                    DropzoneArea {
                        DropzoneIcon { Upload { class: "size-7" } }
                        DropzoneLabel { "Drop a video to visualise the pipeline" }
                        DropzoneHint { "mp4, mov, mkv, webm…" }
                    }
                    VideoWorkflowCanvas {}
                }
            }
        }
    }
}
