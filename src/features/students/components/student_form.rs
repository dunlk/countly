use leptos::prelude::*;

#[component]
pub fn student_form(
    first_name: ReadSignal<String>,
    set_first_name: WriteSignal<String>,

    last_name: ReadSignal<String>,
    set_last_name: WriteSignal<String>,

    editing_id: ReadSignal<Option<i64>>,

    on_save: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="p-6 space-y-5 rounded-xl bg-slate-900">
            <div>
                <label class="block text-sm mg-2">"Nombre"</label>
                <input
                    type="text"
                    class="py-3 px-4 w-full rounded-lg outline-none bg-slate-800"
                    prop:value={move || first_name.get()}
                    on:input={move |ev| {
                        set_first_name.set(event_target_value(&ev));
                    }}
                />
            </div>

            <div>
                <label class="block mb-2 text-sm">"Apellido"</label>

                <input
                    type="text"
                    class="py-3 px-4 w-full rounded-lg outline-none bg-slate-800"
                    prop:value={move || last_name.get()}
                    on:input={move |ev| {
                        set_last_name.set(event_target_value(&ev));
                    }}
                />
            </div>

            <button
                class="py-3 px-4 w-full font-semibold bg-blue-600 rounded-lg"
                on:click={move |_| {
                    on_save.run(());
                }}
            >
                {move || {
                    if editing_id.get().is_some() { "Guardar cambios" } else { "Guardar alumno" }
                }}
            </button>

            <Show when=move || editing_id.get().is_some()>
                <button
                    class="w-full rounded-lg bg-slate-700 px-4 py-3"
                    on:click=move |_| {
                        on_cancel.run(());
                    }>
                    "Cancelar edicion"
                </button>
                </Show>

        </div>
    }
}
