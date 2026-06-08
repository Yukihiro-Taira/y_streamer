use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Destructive,
    Outline,
    Success,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "border-transparent shadow bg-primary text-primary-foreground hover:bg-primary/80"
            }
            BadgeVariant::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            BadgeVariant::Muted => {
                "border-transparent bg-muted text-muted-foreground hover:bg-muted/80"
            }
            BadgeVariant::Destructive => {
                "border-transparent shadow bg-destructive text-destructive-foreground hover:bg-destructive/80"
            }
            BadgeVariant::Outline => "text-foreground",
            BadgeVariant::Success => {
                "border-transparent bg-green-100 text-green-800 hover:bg-green-100/80"
            }
        }
    }
}

#[component]
pub fn Badge(
    #[props(into, optional)] class: Option<String>,
    #[props(default = BadgeVariant::default())] variant: BadgeVariant,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center font-semibold rounded-md border px-2.5 py-0.5 text-xs transition-colors w-fit",
        variant.as_str(),
        class.as_deref().unwrap_or("")
    );
    rsx! { span { class: "{merged}", {children} } }
}
