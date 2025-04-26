use chrono::{DateTime, TimeZone, Utc};
use console_error_panic_hook;
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
fn AgeText(
    birthdate: DateTime<Utc>,
    #[prop(default = String::from(""))] style: String,
    #[prop(default = String::from(""))] class: String,
) -> impl IntoView {
    let age: u32 = Utc::now().years_since(birthdate).unwrap_or(0);

    view! {
        <span style=style class=class>
            {age}
        </span>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-ctp-base text-ctp-flamingo grid grid-cols-2 text-center p-3">
            <p>lmao</p>
            <p>lmao</p>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-ctp-flamingo text-ctp-surface2 grid grid-cols-3 text-center p-3 font-bold">
            <a href="/meow">meow</a>
            <p>lol</p>
            <p>gay</p>
        </footer>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="p2">
            <p>Hello lol</p>
            <p>
                "age: "
                <AgeText
                    birthdate=Utc.with_ymd_and_hms(2006, 10, 31, 0, 0, 0).unwrap()
                    class="".to_string()
                />
            </p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main class="bg-gradient-to-b from-ctp-base to-ctp-crust text-ctp-text flex-grow p-2">
                <Routes fallback=|| "Not found.">
                    <Route path=path!("") view=Home></Route>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| {
        view! { <App /> }
    })
}
