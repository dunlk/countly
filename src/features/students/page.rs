use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use crate::features::students::model::Student;
use crate::tauri::commands::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateStudentArgs {
    first_name: String,
    last_name: String,
}

#[component]
pub fn StudentPage() -> impl IntoView {
    let (students, set_students) = signal(Vec::<Student>::new());
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    // Effect hace que al iniciar el aplicativo se ejecute primero esta peticion
    Effect::new(move |_| {
        spawn_local(async move {
            // JsFuture covierte la Promesa de JavaScript en algo que Rust pueda esperar
            let result = JsFuture::from(invoke("get_students", js_sys::Object::new().into())).await;

            match result {
                // transforma el valor javascript a un <Vec<Student>>
                Ok(value) => match serde_wasm_bindgen::from_value::<Vec<Student>>(value) {
                    Ok(data) => {
                        set_students.set(data);
                    }

                    Err(error) => {
                        web_sys::console::error_1(
                            &format!("Error deserealizando alumnos: {error}").into(),
                        );
                    }
                },
                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        });
    });

    let save_student = move |_| {
        let first_name = first_name.get();
        let last_name = last_name.get();

        spawn_local(async move {
            let student = CreateStudentArgs {
                first_name,
                last_name,
            };

            // serde_wasm_bindgen serializa dato de un studiante
            let student = match serde_wasm_bindgen::to_value(&student) {
                Ok(value) => value,
                Err(error) => {
                    web_sys::console::error_1(&format!("Error serializando: {error}").into());
                    return;
                }
            };

            // wasm_bindgen_futures conexion al back y al comando create_student pasando datos serializados
            let result =
                wasm_bindgen_futures::JsFuture::from(invoke("create_student", student)).await;

            match result {
                Ok(value) => match serde_wasm_bindgen::from_value::<Student>(value) {
                    Ok(student) => {
                        // actualiza y pushea el nuevo estudiante creado
                        // para que aparezca automaticamente en la lista
                        set_students.update(|students| {
                            students.push(student);
                        });

                        // hace que los inputs esten vacios luego de crear un alumno
                        set_first_name.set(String::new());
                        set_last_name.set(String::new());
                    }

                    Err(error) => {
                        web_sys::console::error_1(
                            &format!("Error deserealizando Alumno: {error}").into(),
                        );
                    }
                },

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        });
    };

    view! {
        <main class="min-h-screen pt-[50px] bg-slate-800 text-white p-8">
            <div class="mx-auto max-w-xl">
                <h1 class="mb-8 text-3xl font-bold">
                    "Alumnos"
                </h1>

                <div class="space-y-5 rounded-xl bg-slate-900 p-6">
                    <div>
                        <label class="mb-2 block text-sm">"Nombre"</label>

                        <input
                            type="text"
                            prop:value=move || first_name.get()
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
                            prop:value=move|| last_name.get()
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
            // Mostrar alumnos
            <div class="mt-8 space-y-3 bg-">
                <h2 class="text-xl font-semibold">
                    "Alumnos registragdos"
                </h2>

                <For
                    each=move || students.get()
                    key=|student| student.id
                    children=move |student| {
                        view! {
                        <div class="rounded-lg bg-slate-900 p-4">
                            <p class="font-medium">
                                {format!(
                                    "{}, {}",
                                    student.first_name,
                                    student.last_name
                                )}
                            </p>
                        </div>
                    }
                    }
                />
            </div>
        </main>
    }
}
