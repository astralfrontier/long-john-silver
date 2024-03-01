use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <div class:container=true>
            <button
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }

                class:button=true
                class=("is-primary", move || { count() < 25 })
                disabled=move || { count() > 24 }
            >
                // on stable, this is move || count.get();
                "Clickin' chicken: "
                {move || count()}
            </button>
            <button
                on:click=move |_| { set_count(0) }

                class:button=true
                class=("is-primary", move || { count() > 24 })
            >
                "Reset"
            </button>
            <br/>
            <progress
                max="50"
                // signals are functions, so `value=count` and `value=move || count.get()`
                // are interchangeable.
                value=double_count
            ></progress>
        </div>
    }
}
