use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::components::ui::label::Label;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldVariant {
    #[default]
    Vertical,
    Horizontal,
}

#[component]
pub fn FieldGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex flex-col gap-3 w-full", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "FieldGroup", class: "{merged}", {children} }
    }
}

#[component]
pub fn Field(
    #[props(into, optional)] class: Option<String>,
    #[props(default = FieldVariant::default())] variant: FieldVariant,
    #[props(default = false)] disabled: bool,
    children: Element,
) -> Element {
    let layout = match variant {
        FieldVariant::Vertical => "flex-col items-start",
        FieldVariant::Horizontal => "flex-row items-center",
    };
    let merged = tw_merge!(
        "flex gap-1.5 rounded-lg border bg-background px-3 py-3 w-full",
        layout,
        if disabled { "opacity-60" } else { "" },
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "Field",
            "data-disabled": if disabled { "true" } else { "false" },
            class: "{merged}",
            {children}
        }
    }
}

#[component]
pub fn FieldLabel(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] html_for: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "text-[11px] uppercase tracking-wide text-muted-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        Label { html_for: html_for, class: "{merged}", {children} }
    }
}

#[component]
pub fn FieldDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-xs text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        p { "data-name": "FieldDescription", class: "{merged}", {children} }
    }
}
