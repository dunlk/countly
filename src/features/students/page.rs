use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use crate::features::students::components::student_form::StudentForm;
use crate::features::students::components::student_list::StudentList;
use crate::features::students::model::Student;
use crate::tauri::commands::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateStudentArgs {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
struct DeleteStudentArgs {
    id: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateStudentArgs {
    id: i64,
    first_name: String,
    last_name: String,
}

#[component]
pub fn StudentPage() -> impl IntoView {
    let (students, set_students) = signal(Vec::<Student>::new());
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    // stados o senales para capturar el id y su valor inicial puede ser un i64 o None
    let (editing_id, set_editing_id) = signal(Option::<i64>::None);

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

    let save_student = move || {
        let first_name = first_name.get();
        let last_name = last_name.get();
        let editing_id = editing_id.get();

        spawn_local(async move {
            match editing_id {
                Some(id) => {
                    let args = UpdateStudentArgs {
                        id,
                        first_name,
                        last_name,
                    };

                    let args = match serde_wasm_bindgen::to_value(&args) {
                        Ok(value) => value,

                        Err(error) => {
                            web_sys::console::error_1(
                                &format!("Error serializando: {error}").into(),
                            );
                            return;
                        }
                    };

                    let result = JsFuture::from(invoke("update_student", args)).await;

                    match result {
                        Ok(value) => match serde_wasm_bindgen::from_value::<Student>(value) {
                            Ok(updated_student) => {
                                set_students.update(|students| {
                                    // Some es un si estudiante existe
                                    if let Some(student) = students
                                        // iter_mut nor permite iterar y mutar los valores
                                        .iter_mut()
                                        .find(|student| student.id == updated_student.id)
                                    {
                                        *student = updated_student;
                                    }
                                });

                                set_editing_id.set(None);
                                set_first_name.set(String::new());
                                set_last_name.set(String::new());
                            }

                            Err(error) => {
                                web_sys::console::error_1(
                                    &format!("Error deserializando: {error}").into(),
                                );
                            }
                        },

                        Err(error) => {
                            web_sys::console::error_1(&error);
                        }
                    }
                }
                None => {
                    let student = CreateStudentArgs {
                        first_name,
                        last_name,
                    };

                    // serde_wasm_bindgen serializa dato de un studiante
                    let student = match serde_wasm_bindgen::to_value(&student) {
                        Ok(value) => value,
                        Err(error) => {
                            web_sys::console::error_1(
                                &format!("Error serializando: {error}").into(),
                            );
                            return;
                        }
                    };

                    // wasm_bindgen_futures conexion al back y al comando create_student pasando datos serializados
                    let result =
                        wasm_bindgen_futures::JsFuture::from(invoke("create_student", student))
                            .await;

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
                }
            }
        });
    };

    let on_save = Callback::new(move |_| {
        save_student();
    });

    let on_cancel = Callback::new(move |_| {
        set_editing_id.set(None);
        set_first_name.set(String::new());
        set_last_name.set(String::new());
    });

    let on_delete = Callback::new(move |student_id: i64| {
        spawn_local(async move {
            let args = DeleteStudentArgs { id: student_id };

            let args = match serde_wasm_bindgen::to_value(&args) {
                Ok(value) => value,

                Err(error) => {
                    web_sys::console::error_1(&format!("Error serializando: {error}").into());
                    return;
                }
            };

            let result = JsFuture::from(invoke("delete_student", args)).await;

            match result {
                Ok(_) => {
                    set_students.update(|students| {
                        students.retain(|student| student.id != student_id);
                    });
                }

                Err(error) => {
                    web_sys::console::error_1(&error);
                }
            }
        });
    });

    let on_edit = Callback::new(move |student: Student| {
        set_editing_id.set(Some(student.id));

        set_first_name.set(student.first_name);
        set_last_name.set(student.last_name);
    });

    view! {
        <main class="p-8 min-h-screen text-white pt-[50px] bg-slate-800">
            // formulario estudiante
            <StudentForm
                    first_name=first_name
                    set_first_name=set_first_name
                    last_name=last_name
                    set_last_name=set_last_name
                    editing_id=editing_id
                    on_save=on_save
                    on_cancel=on_cancel
                />
            // Mostrar alumnos
            <StudentList students={students} on_edit={on_edit} on_delete={on_delete} />
        </main>
    }
}
