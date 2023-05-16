use leptos::*;
use leptos::ev::{KeyboardEvent, MouseEvent, Event};
use leptos::html::Input;
use log::info;
use crate::api;
use std::str::FromStr;
use trusty_gardener_model::Plant;
use crate::components::{Footer, PlantView};

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    let plants = create_resource(cx, || (), |_| async move { api::get_plants().await.unwrap_or(Vec::new())});

    let (selected_plant, set_selected_plant) = create_signal(cx, None);

    create_effect(cx, move |_| { // TODO: Remove effect when we know how to derive the two signals!
        if selected_plant.get().is_none() {
            if let Some(plants) = plants.read(cx) {
                set_selected_plant.set(plants.get(0).cloned())
            }
        }
    });

    let (show_create_plant_modal, set_show_create_plant_modal) = create_signal(cx, false);
    let show_create_plant_modal_handler = move |_: MouseEvent| {
        set_show_create_plant_modal.set(true);
    };
    let (create_plant_modal_name, set_plant_modal_name) = create_signal(cx, String::new());

    let create_plant_modal_name_input: NodeRef<Input> = create_node_ref(cx);

    view! { cx,
        <div class="container hero is-fluid is-fullheight">
            <div class={move || if show_create_plant_modal.get() {"modal is-active"} else {"modal"}}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Create new plant"}</p>
                    </header>
                    <section class="modal-card-body">
                        <div class="field">
                            <label class="label">{"Name:"}</label>
                            <div class="control">
                                <input node_ref=create_plant_modal_name_input class="input" type="text" placeholder={"e.g Silver Birch"}/>
                            </div>
                        </div>
                        <div class="field">
                            <label class="label">{"Species:"}</label>
                            <div class="control">
                                <input class="input" type="text" placeholder={"e.g betula pendula"}/>
                            </div>
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-success" on:click=move |_| {
                            set_show_create_plant_modal.set(false);
                            let name = create_plant_modal_name_input
                                .get()
                                .expect("name input element")
                                .value();
                            info!("{}", name);
                        }>{"Create"}</button>
                        <button class="button" on:click=move |_| set_show_create_plant_modal.set(false)>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
            <div class="title is-1">
                <h1 id={ "projectName" }>
                    <span class={ "blackName" }>{ "T" }</span>
                    <span class={ "redName" }>{ "RUST" }</span>
                    <span class={ "blackName" }>{ "Y GARDENER" }</span>
                </h1>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <article class="tile p-2 is-child box">
                            <div class="select is-medium">
                                <select on:change=move |event: Event| {
                                    if let Some(plants) = plants.read(cx) {
                                        set_selected_plant.set(plants.get(usize::from_str(&event_target_value(&event)).unwrap()).cloned());
                                    };
                                }>
                                    {
                                        move || match plants.read(cx) {
                                            Some(plants) => {
                                                plants.into_iter()
                                                    .enumerate()
                                                    .map(|(index, plant) | view! {cx, <option value=index>{plant.name}</option>})
                                                    .collect::<Vec<_>>()
                                            },
                                            None => Vec::new()
                                        }
                                    }
                               </select>
                            </div>
                            <button class="button is-medium" on:click=show_create_plant_modal_handler>
                                    <i class="fa-solid fa-plus"></i>
                            </button>
                            <button class="button is-medium subtitle">
                                <i class="fa-solid fa-arrow-up-from-bracket"></i>
                            </button>
                        </article>
                    </div>
                </div>
            </div>
            {move || {
                view!{cx, <div><PlantView plant={selected_plant}></PlantView></div>}
            }}
            <Footer></Footer>
        </div>
    }
}
