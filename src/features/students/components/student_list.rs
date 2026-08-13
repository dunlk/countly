use leptos::prelude::*;

use crate::features::students::model::Student;

#[component]
pub fn StudentList(
    students: ReadSignal<Vec<Student>>,
    on_edit: Callback<Student>,
    on_delete: Callback<i64>,
) -> impl IntoView {
    view! {
        <div class="mt-8 space-y-3">
            <h2 class="text-xl font-semibold">Alumnos registrados</h2>

            <For
                each={move || students.get()}
                key={|student| student.id}
                children={move |student| {
                    let student_id = student.id;
                    let student_for_edit = student.clone();

                    view! {
                        <div class="flex justify-between items-center p-4 rounded-lg bg-slate-900">
                            <p class="font-semibold">
                                {format!("{} {}", student.first_name, student.last_name)}
                            </p>
                            <div class="flex gap-2">
                                <button
                                    class="py-2 px-3 text-sm rounded-lg bg-slate-700"
                                    on:click={move |_| {
                                        on_edit.run(student_for_edit.clone());
                                    }}
                                >
                                    "Editar"
                                </button>

                                <button
                                    class="py-2 px-3 text-sm font-semibold bg-red-600 rounded-lg"
                                    on:click={move |_| {
                                        on_delete.run(student_id);
                                    }}
                                >
                                    "Eliminar"
                                </button>
                            </div>
                        </div>
                    }
                }}
            />
        </div>
    }
}
