use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn ContextMenuLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("px-2 py-1.5 text-sm font-medium data-inset:pl-8 mb-1", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "ContextMenuLabel", class: "{merged}", {children} } }
}

#[component]
pub fn ContextMenuGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! { ul { "data-name": "ContextMenuGroup", class: "{merged}", {children} } }
}
