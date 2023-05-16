use leptos::ev::MouseEvent;
use leptos::*;
use gloo_timers::callback::Timeout;
use trusty_gardener_model::{Plant, PlantWateringHistory};

use crate::api;

const WATERING_CAN_STATIC: &'static str = "wateringcan.png";
const WATERING_CAN_ANIMATED: &'static str = "wateringcan3.gif";

#[component]
pub fn PlantView(cx: Scope, plant: ReadSignal<Option<Plant>>) -> impl IntoView {

    let (watering_icon, set_watering_icon) = create_signal(cx, WATERING_CAN_STATIC);

    let watering_history = create_resource(cx, move || plant.get(), |plant| async move {
        if let Some(plant) = plant {
            api::get_watering_history(plant.name).await.unwrap_or(PlantWateringHistory::default())
        }
        else {
            PlantWateringHistory::default()
        }
    });

    let watering_action = {
        move |_: MouseEvent| {
            if let Some(plant) = plant.get() {
                set_watering_icon.set(WATERING_CAN_ANIMATED);
                spawn_local(async move {
                    api::do_watering(plant.name).await;
                    watering_history.refetch();
                });
                {
                    Timeout::new(3000, move || {
                        set_watering_icon.set(WATERING_CAN_STATIC);
                    }).forget();
                }
            }
        }
    };

    let clear_watering_history = move |_: MouseEvent| {
        if let Some(plant) = plant.get() {
            spawn_local(async move {
                api::clear_watering_history(plant.name).await;
                watering_history.refetch();
            });
        }
    };

    view! { cx,
        {
            move || {
                if let Some(plant) = plant.get() {
                    view!{cx,
                        <div class="columns">
                            <div class="column is-two-thirds">
                                <div class="subtitle is-3">{"Name: "}{plant.name}</div>
                                <div class="tile is-ancestor">
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <p class="title is-4">{"Wasserstand"}</p>
                                        </article>
                                    </div>
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <p class="title is-4">{"Helligkeit"}</p>
                                        </article>
                                    </div>
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <p class="title is-4">{"Gießen"}</p>
                                            <p class="subtitle is-6">{ "Drücke auf das Bild, wenn du die Pflanze gegossen hast, um die Daten zu speichern:" }</p>
                                            {
                                                if let Some(watering_history) = &watering_history.read(cx) {
                                                    let count = watering_history.history.len();
                                                    view!{ cx, <p>{move || { count }}</p> }
                                                }
                                                else {
                                                    view!{ cx, <p>{"<< error >>"}</p>}
                                                }
                                            }
                                            <img on:click=watering_action src={watering_icon.get()} alt="watering" title="watering can" width="100" height="100"/>
                                        </article>
                                    </div>
                                </div>
                                <div class="tile is-ancestor">
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <p class="title is-4">{"History"}</p>
                                            <button class="button" on:click=clear_watering_history>{ "Clear" }</button>
                                        </article>
                                    </div>
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <p class="title is-4">{"optional"}</p>
                                        </article>
                                    </div>
                                </div>
                            </div>
                            <div class="column">
                                <div class="tile is-ancestor">
                                    <div class="tile is-parent">
                                        <article class="tile is-child box">
                                            <img src="plant1.png" alt="a plant" />
                                        </article>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }
                else {
                    view! {cx, <div></div>}
                }
            }
        }

    }
}
