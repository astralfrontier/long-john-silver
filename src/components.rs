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
    opened_by: String,
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
        opened_by: String::from("Unknown"),
        floors: 0,
    }]);

    view! {
        <div class:container=true>
            <div class:columns=true>
                <div class:column=true class=("is-6", true)>
                    <div class:columns=true>
                        <div class:column=true class=("is-6", true)><strong>Player</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Maps</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Actions</strong></div>
                    </div>
                    <For
                        each=users
                        key=|map_user| format!("{}-{}-{}-{}", map_user.id.clone(), map_user.name.clone(), map_user.maps_opened.clone(), map_user.maps_held.clone())
                        let:map_user
                    >
                        <div class:columns=true class=("is-vcentered", true)>
                            <div class:column=true class=("is-6", true)>
                                <nav class:level=true>
                                    <div class=("level-left", true)>
                                        <div class=("level-item", true)>
                                            <button
                                                class:delete=true
                                                class=("is-small", true)
                                                on:click=move |_| {
                                                    set_users.update(|users| {
                                                        users.retain(|u| u.id != map_user.id)
                                                    })
                                                }
                                            ></button>
                                        </div>
                                        <div class=("level-item", true)>
                                            {map_user.name.clone()}
                                        </div>
                                    </div>
                                </nav>
                            </div>
                            <div class:column=true class=("is-3", true)>{map_user.maps_opened}/{map_user.maps_held}</div>
                            <div class:column=true class=("is-3", true)>
                                <button
                                    class:button=true
                                    class=("is-small", true)
                                    class=("is-link", true)
                                    on:click=move |_| {
                                        set_portals.update(|portals| portals.push(MapPortal {
                                            id: portals.iter().fold(std::i32::MIN, |a,b| a.max(b.id)) + 1,
                                            name: String::from("New Portal"),
                                            opened_by: map_user.name.clone(),
                                            floors: 0
                                        }))
                                    }
                                >
                                    Portal
                                </button>
                            </div>
                        </div>
                    </For>
                    <button
                        class:button=true
                        class=("is-primary", true)
                        on:click=move |_| {
                            set_users.update(|users| users.push(MapUser {
                                id: users.iter().fold(std::i32::MIN, |a,b| a.max(b.id)) + 1,
                                name: String::from("New Player"),
                                maps_opened: 0,
                                maps_held: 0
                            }));
                        }
                    >
                        Add Player
                    </button>
                </div>
                <div class:column=true class=("is-6", true)>
                    <div class:columns=true>
                        <div class:column=true class=("is-3", true)><strong>Portal</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Opened By</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Floors</strong></div>
                        <div class:column=true class=("is-3", true)><strong>Actions</strong></div>
                    </div>
                    <For
                        each=portals
                        key=|portal| format!("{}-{}-{}", portal.id.clone(), portal.name.clone(), portal.floors.clone())
                        let:portal
                    >
                        <div class:columns=true>
                            <div class:column=true class=("is-3", true)>
                            <nav class:level=true>
                            <div class=("level-left", true)>
                                <div class=("level-item", true)>
                                    <button
                                        class:delete=true
                                        class=("is-small", true)
                                        on:click=move |_| {
                                            set_portals.update(|portals| {
                                                portals.retain(|p| p.id != portal.id)
                                            })
                                        }
                                    ></button>
                                </div>
                                <div class=("level-item", true)>
                                    {portal.name.clone()}
                                </div>
                            </div>
                        </nav>
                            </div>
                            <div class:column=true class=("is-3", true)>{portal.opened_by}</div>
                            <div class:column=true class=("is-3", true)>{portal.floors}</div>
                            <div class:column=true class=("is-3", true)>
                            </div>
                        </div>
                    </For>
                </div>
            </div>
        </div>
    }
}
