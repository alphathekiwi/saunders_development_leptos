use leptos::*;
use stylers::style;


fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
#[component]
fn App() -> impl IntoView {

    let (count, set_count) = create_signal("John");
    view! {
        <p>{move || count()}</p>
        <Hello name={count.get()}/>
        <button on:click=move |_| {set_count("Mary");}> "Click me!" </button>
    }
}
#[component]
fn Hello(name: &'static str) -> impl IntoView {
    let styler_class = style! {"hello_world",
        #two{
            color: blue;
        }
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };

    view! {class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>{name}</h2>
            <h3>"Hello Lizzy"</h3>
        </div>
    }
}