use leptos::*;
use leptos::ev::{KeyboardEvent, MouseEvent, Event};
use leptos::html::Input;
use log::info;
use crate::api;
use std::str::FromStr;
use trusty_gardener_model::Plant;
use crate::components::{Footer, PlantView, PlantEditor};

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

    let plant_editor_visible: RwSignal<bool> = create_rw_signal(cx, false);
    let plant_editor_plant: RwSignal<Option<Plant>> = create_rw_signal(cx, None);

    view! { cx,
        <div class="container hero is-fluid is-fullheight">
            <PlantEditor visible=plant_editor_visible plant=plant_editor_plant></PlantEditor>
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
                            <button class="button is-medium" on:click=move |_| {
                                plant_editor_visible.set(true);
                                plant_editor_plant.set(None);
                            }>
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
                let plant_editor_visible = plant_editor_visible.write_only();
                let plant_editor_plant = plant_editor_plant.write_only();
                view!{cx, <div><PlantView plant={selected_plant} plant_editor_visible=plant_editor_visible plant_editor_plant=plant_editor_plant></PlantView></div>}
            }}
            <Footer></Footer>
        </div>
    }
}
