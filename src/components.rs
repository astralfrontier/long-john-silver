use leptos::*;

#[derive(Debug, Clone)]
struct MapUser {
    id: i32,
    name: String,
    maps_opened: i32,
    maps_held: i32,
}

#[derive(Debug, Clone)]
struct MapPortal {
    id: i32,
    name: String,
    floors: i32,
}

#[component]
pub fn MapUserView() -> impl IntoView {}

#[component]
pub fn App() -> impl IntoView {
    let (users, set_users) = create_signal(vec![MapUser {
        id: 0,
        name: String::from("New User"),
        maps_opened: 0,
        maps_held: 0,
    }]);
    let (portals, set_portals) = create_signal(vec![MapPortal {
        id: 0,
        name: String::from("New Portal"),
        floors: 0,
    }]);

    view! {
        <div class:container=true>
            <div class:columns=true>
                <div class:column=true class=("is-6", true)>
                    <div class:columns=true>
                        <div class:column=true class=("is-6", true)><strong>Player</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Maps</strong></div>
                    </div>
                    <For
                        each=users
                        key=|map_user| format!("{}-{}-{}-{}", map_user.id.clone(), map_user.name.clone(), map_user.maps_opened.clone(), map_user.maps_held.clone())
                        let:map_user
                    >
                        <div class:columns=true>
                            <div class:column=true class=("is-8", true)>{map_user.name}</div>
                            <div class:column=true class=("is-4", true)>{map_user.maps_opened}/{map_user.maps_held}</div>
                        </div>
                    </For>
                </div>
                <div class:column=true class=("is-6", true)>
                    <div class:columns=true>
                        <div class:column=true class=("is-8", true)><strong>Portal</strong></div>
                        <div class:column=true class=("is-4", true)><strong>Floors</strong></div>
                    </div>
                    <For
                        each=portals
                        key=|portal| format!("{}-{}-{}", portal.id.clone(), portal.name.clone(), portal.floors.clone())
                        let:portal
                    >
                        <div class:columns=true>
                            <div class:column=true class=("is-8", true)>{portal.name}</div>
                            <div class:column=true class=("is-4", true)>{portal.floors}</div>
                        </div>
                    </For>
                </div>
            </div>
        </div>
    }
}
