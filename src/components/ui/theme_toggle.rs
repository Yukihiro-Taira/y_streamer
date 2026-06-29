use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Moon, Sun};

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let mut ev = eval(
                "return localStorage.getItem('darkmode') === 'true' || \
                 (localStorage.getItem('darkmode') === null && \
                  window.matchMedia('(prefers-color-scheme: dark)').matches);",
            );
            if let Ok(is_dark) = ev.recv::<bool>().await {
                dark.set(is_dark);
                if is_dark {
                    eval("document.documentElement.classList.add('dark')");
                }
            }
        });
    });

    use_effect(move || {
        let is_dark = dark();
        eval(&format!(
            "document.documentElement.classList.toggle('dark', {is_dark}); \
             localStorage.setItem('darkmode', '{is_dark}');"
        ));
    });

    rsx! {
        button {
            r#type: "button",
            "aria-label": "Toggle theme",
            class: "inline-flex items-center justify-center size-8 rounded-md hover:bg-accent transition-colors cursor-pointer",
            onclick: move |_| dark.set(!dark()),
            if dark() {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
        }
    }
}
