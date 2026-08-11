use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

use crate::tauri::commands::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateStudentArgs {
    first_name: String,
    last_name: String,
}

#[component]
pub fn StudentPage() -> impl IntoView {
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    let save_student = move |_| {
        web_sys::console::log_1(&"CLICK GUARDAR".into());

        let first_name = first_name.get();
        let last_name = last_name.get();

        spawn_local(async move {
            let args = CreateStudentArgs {
                first_name,
                last_name,
            };

            let args = match serde_wasm_bindgen::to_value(&args) {
                Ok(value) => value,
                Err(error) => {
                    web_sys::console::error_1(&format!("Error serializando: {error}").into());
                    return;
                }
            };

            let result = wasm_bindgen_futures::JsFuture::from(invoke("create_student", args)).await;

            match result {
                Ok(value) => {
                    web_sys::console::log_1(&value);
                }

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        });
    };

    view! {
        <main class="min-h-screen bg-slate-950 text-white p-8">
            <div class="mx-auto max-w-xl">
                <h1 class="mb-8 text-3xl font-bold">
                    "Alumnos"
                </h1>

                <div class="space-y-5 rounded-xl bg-slate-900 p-6">
                    <div>
                        <label class="mb-2 block text-sm">"Nombre"</label>

                        <input
                            type="text"
                            class="w-full rounded-lg bg-slate-800 px-4 py-3 outline-none"
                            on:input=move |ev| {
                                set_first_name.set(event_target_value(&ev));
                            }
                        />
                    </div>

                    <div>
                        <label class="mb-2 block text-sm">
                            "Apellido"
                        </label>

                        <input
                            type="text"
                            class="w-full rounded-lg bg-slate-800 px-4 py-3 outline-none"
                            on:input=move |ev| {
                                set_last_name.set(event_target_value(&ev));
                            }
                        />
                    </div>

                    <button
                        class="w-full rounded-lg bg-blue-600 px-4 py-3 font-semibold"
                        on:click=save_student
                    >
                        "Guardar alumno"
                    </button>
                </div>
            </div>
        </main>
    }
}
