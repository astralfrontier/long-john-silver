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
    let empty_users: Vec<MapUser> = Vec::new();
    let empty_portals: Vec<MapPortal> = Vec::new();
    let (users, set_users) = create_signal(empty_users);
    let (portals, set_portals) = create_signal(empty_portals);

    view! {
        <div class:container=true>
            <div class:columns=true>
                <div class:column=true class=("is-6", true)>
                    <div class:columns=true>
                        <div class:column=true class=("is-6", true)><strong>Player</strong></div>
                        <div class:column=true class=("is-2", true)><strong>Opened</strong></div>
                        <div class:column=true class=("is-2", true)><strong>Held</strong></div>
                        <div class:column=true class=("is-2", true)><strong>Actions</strong></div>
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
                                            <input
                                                class:input=true
                                                prop:type="text"
                                                prop:value=&map_user.name
                                                on:change=move |ev| {
                                                    set_users.update(|users| {
                                                        for u in users.iter_mut() {
                                                            if u.id == map_user.id {
                                                                u.name = event_target_value(&ev)
                                                            }
                                                        }
                                                    })
                                                }
                                            ></input>
                                        </div>
                                    </div>
                                </nav>
                            </div>
                            <div class:column=true class=("is-2", true)>
                                <input class:input=true
                                    prop:type="number"
                                    prop:size="4"
                                    prop:value=&map_user.maps_opened.to_string()
                                    on:change=move |ev| {
                                        set_users.update(|users| {
                                            for u in users.iter_mut() {
                                                if u.id == map_user.id {
                                                    u.maps_opened = match event_target_value(&ev).parse::<i32>() {
                                                        Ok(n) => n,
                                                        Err(_e) => u.maps_opened
                                                    }
                                                }
                                            }
                                        })
                                    }
                                ></input>
                            </div>
                            <div class:column=true class=("is-2", true)>
                                <input class:input=true
                                    prop:type="number"
                                    prop:size="4"
                                    prop:value=&map_user.maps_held.to_string()
                                    on:change=move |ev| {
                                        set_users.update(|users| {
                                            for u in users.iter_mut() {
                                                if u.id == map_user.id {
                                                    u.maps_held = match event_target_value(&ev).parse::<i32>() {
                                                        Ok(n) => n,
                                                        Err(_e) => u.maps_held
                                                    }
                                                }
                                            }
                                        })
                                    }
                                ></input>
                            </div>
                            <div class:column=true class=("is-2", true)>
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
                    <div class:columns=true class=("is-vcentered", true)>
                        <div class:column=true class=("is-5", true)><strong>Portal</strong></div>
                        <div class:column=true class=("is-5", true)><strong>Opened By</strong></div>
                        <div class:column=true class=("is-2", true)><strong>Floors</strong></div>
                    </div>
                    <For
                        each=portals
                        key=|portal| format!("{}-{}-{}", portal.id.clone(), portal.name.clone(), portal.floors.clone())
                        let:portal
                    >
                        <div class:columns=true>
                            <div class:column=true class=("is-5", true)>
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
                                        <input
                                            class:input=true
                                            prop:type="text"
                                            prop:value=&portal.name
                                            on:change=move |ev| {
                                                set_portals.update(|portals| {
                                                    for u in portals.iter_mut() {
                                                        if u.id == portal.id {
                                                            u.name = event_target_value(&ev)
                                                        }
                                                    }
                                                })
                                            }
                                        ></input>
                                    </div>
                                </div>
                            </nav>
                            </div>
                            <div class:column=true class=("is-5", true)>{portal.opened_by}</div>
                            <div class:column=true class=("is-2", true)>
                                <input class:input=true
                                    prop:type="number"
                                    prop:size="4"
                                    prop:value=&portal.floors.to_string()
                                    on:change=move |ev| {
                                        set_portals.update(|portals| {
                                            for p in portals.iter_mut() {
                                                if p.id == portal.id {
                                                    p.floors = match event_target_value(&ev).parse::<i32>() {
                                                        Ok(n) => n,
                                                        Err(_e) => p.floors
                                                    }
                                                }
                                            }
                                        })
                                    }
                                ></input>
                            </div>
                            <div class:column=true class=("is-3", true)>
                            </div>
                        </div>
                    </For>
                </div>
            </div>
        </div>
    }
}
