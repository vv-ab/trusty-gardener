use leptos::*;
use leptos::ev::MouseEvent;
use leptos::html::Input;
use trusty_gardener_model::Plant;
use log::info;

#[component]
pub fn PlantEditor(cx: Scope, visible: RwSignal<bool>, plant: RwSignal<Option<Plant>>) -> impl IntoView {

    let (show_create_plant_modal, set_show_create_plant_modal) = create_signal(cx, false);
    let show_create_plant_modal_handler = move |_: MouseEvent| {
        set_show_create_plant_modal.set(true);
    };
    let (create_plant_modal_name, set_plant_modal_name) = create_signal(cx, String::new());

    let create_plant_modal_name_input: NodeRef<Input> = create_node_ref(cx);
    view! { cx,
         <div class={move || if visible.get() {"modal is-active"} else {"modal"}}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Create new plant"}</p>
                        <p>{move || plant.get().map_or(String::new(), |plant| plant.name)}</p>
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
                        <button class="button" on:click=move |_| visible.set(false)>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
    }
}