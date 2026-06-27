use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_params_map;

use crate::models::Category;
use crate::server::*;

#[component]
pub fn MoveLibrary() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    let (move_to_delete, set_move_to_delete) = signal::<Option<i32>>(None);
    let (refresh_trigger, set_refresh_trigger) = signal(0);
    
    view! {
        <div class="library-container">
            <div class="techniques">
                <TechniqueTable 
                    show_add_modal=set_show_modal 
                    show_confirm_modal=set_move_to_delete
                    refresh_trigger=refresh_trigger
                    />
                <Show when=move || show_modal.get()>
                    <AddModal 
                        show_add_modal=set_show_modal 
                        set_refresh_trigger=set_refresh_trigger
                        />
                </Show>
                <Show when=move || move_to_delete.get().is_some()>
                    <ConfirmationModal 
                        id_to_delete=move_to_delete 
                        show_confirm_modal=set_move_to_delete
                        set_refresh_trigger=set_refresh_trigger
                        />
                </Show>
            </div>
            <div class="info">
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
pub fn TechniqueTable(
    show_add_modal: WriteSignal<bool>, 
    show_confirm_modal: WriteSignal<Option<i32>>,
    refresh_trigger: ReadSignal<i32>,
) -> impl IntoView {

    let moves_resource = Resource::new(
        move || refresh_trigger.get(),
        |_| async move { get_moves().await }
    );

    view! {            
        <div class="library-header">
            <h2>"Technique Library"</h2>
            <button class="btn btn-add" on:click=move |_| show_add_modal.set(true)>
                <svg fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="add-icon">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
                "Add Technique"
            </button>
        </div>
        <Suspense fallback=|| view! { <p class="loading">"Loading techniques..."</p> }>
            <ErrorBoundary fallback=|_| view! { 
                <div class="error-box">
                    <p>"Whoops! Failed to load the database."</p>
                </div> 
            }>
                {move || moves_resource.get().map(|data| {
                    
                    let moves = data.unwrap(); 
                    
                    if moves.is_empty() {
                        view! { <p>"No moves found. Add a technique to get started!"</p> }.into_any()
                    } else {
                        view! {
                            <div class="move-header">
                                <div class="move-item">
                                    <div class="move-category">Category</div>
                                    <div class="move-name">Name</div>
                                    <div class="move-link">Link</div>
                                    <div class="move-details"></div>
                                    <div class="move-edit"></div>
                                    <div class="move-delete"></div>
                                </div>
                            </div>
                            <div class="move-list">
                                {moves.into_iter().map(|m| view! {
                                    <div class="move-item">
                                        <div class="move-category">{m.category}</div>
                                        <div class="move-name">{m.name}</div>
                                        <div class="move-link">{m.video_link.unwrap_or("none".to_string())}</div>
                                        <a href=format!("/techniques/{}", m.id) class="move-action move-details" title="View Details">
                                            <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                            </svg>
                                        </a>

                                        <a href=format!("/techniques/{}/edit", m.id) class="move-action move-edit" title="Edit Move">
                                            <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                                            </svg>
                                        </a>

                                        <button class="move-action move-delete" title="Delete Move" on:click=move |_| show_confirm_modal.set(Some(m.id))>
                                            <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                            </svg>
                                        </button>
                                    </div>
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn AddModal(show_add_modal: WriteSignal<bool>, set_refresh_trigger: WriteSignal<i32>) -> impl IntoView {
    let add_action = ServerAction::<AddMove>::new();

    Effect::new(move |_| {
        if let Some(Ok(_)) = add_action.value().get() {
            show_add_modal.set(false);
            set_refresh_trigger.update(|n| *n += 1);
            add_action.value().set(None);
        }
    });

    view! {
        <div class="modal-overlay">
            <div class="modal-content move-card">
                <h3>"Add New Technique"</h3>
                
                <ActionForm action=add_action attr:class="add-move-form">
                    <div class="form-group">
                        <label for="new_name">"Technique Name"</label>
                        <input type="text" id="new_name" name="new_name" required placeholder="e.g., Triangle Choke" />
                    </div>
                    
                    <div class="form-group">
                        <label for="new_category">"Category"</label>
                        <select id="new_category" name="new_category">
                            {Category::ALL.into_iter().map(|cat| {
                                let cat_name: String = cat.into();
                                let cat_value = cat_name.to_lowercase();
                                view! {
                                    <option value=cat_value>{cat_name}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="new_difficulty">"Difficulty (1-5)"</label>
                        <input type="number" id="new_difficulty" name="new_difficulty" min="1" max="5" value="1" required />
                    </div>

                    <div class="form-group">
                        <label for="new_video_link">"Video Link (Optional)"</label>
                        <input type="url" id="new_video_link" name="new_video_link" placeholder="YouTube URL" />
                    </div>

                    <div class="form-group">
                        <label for="new_notes">"Study Notes (Optional)"</label>
                        <textarea id="new_notes" name="new_notes" rows="4" placeholder="Key details, grips, common mistakes..."></textarea>
                    </div>

                    <Show when=move || add_action.value().with(|val| matches!(val, Some(Err(_))))>
                        <div class="form-error">
                            {move || add_action.value().with(|val| {
                                if let Some(Err(e)) = val {
                                    e.to_string()
                                } else {
                                    String::new()
                                }
                            })}
                        </div>
                    </Show>

                    <div class="form-action-buttons">
                        <button type="button" class="btn btn-cancel" on:click=move |_| show_add_modal.set(false)>
                            "Cancel"
                        </button>
                        <button type="submit" class="btn btn-submit">
                            "Save Technique"
                        </button>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}

#[component]
pub fn ConfirmationModal(
    id_to_delete: ReadSignal<Option<i32>>, 
    show_confirm_modal: WriteSignal<Option<i32>>,
    set_refresh_trigger: WriteSignal<i32>
) -> impl IntoView {
    let delete_action = ServerAction::<DeleteMoveById>::new();

    Effect::new(move |_| {
        if let Some(Ok(_)) = delete_action.value().get() {
            show_confirm_modal.set(None);
            set_refresh_trigger.update(|n| *n += 1);
            delete_action.value().set(None);
        }
    });

    view!{
        <div class="modal-overlay">
            <div class="modal-content move-card delete-modal-card">
                
                <div class="warning-icon-wrapper">
                    <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="warning-icon">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                </div>

                <h3>"Delete Technique?"</h3>
                <p class="modal-description">
                    "Are you sure you want to delete this technique? This action cannot be undone and will permanently remove it from your library."
                </p>

                <div class="form-action-buttons">
                    <button class="btn btn-cancel" on:click=move |_| show_confirm_modal.set(None)>
                        "Cancel"
                    </button>
                    
                    <button class="btn btn-danger" on:click=move |_| {
                        if let Some(id) = id_to_delete.get() {
                            delete_action.dispatch(DeleteMoveById { move_id: id });
                        }
                    }>
                        "Yes, Delete"
                    </button>
                </div>
            </div>
        </div>
    }
}

fn get_embed_url(raw_url: &str) -> String {
    if raw_url.contains("youtube.com/watch?v=") {
        if let Some(id_part) = raw_url.split("v=").nth(1) {
            let id = id_part.split('&').next().unwrap_or(id_part);
            return format!("https://www.youtube.com/embed/{}", id);
        }
    } else if raw_url.contains("youtu.be/") {
        if let Some(id_part) = raw_url.split("youtu.be/").nth(1) {
            let id = id_part.split('?').next().unwrap_or(id_part);
            return format!("https://www.youtube.com/embed/{}", id);
        }
    }
    raw_url.to_string()
}

#[component]
pub fn MoveDetail() -> impl IntoView {
    let params = use_params_map();
    
    let move_id = move || {
        params.with(|p| p.get("id").and_then(|id| id.parse::<i32>().ok()).unwrap_or(0))
    };

    let move_resource = Resource::new(
        move_id, 
        |id| async move { get_move_by_id(id).await }
    );

    view! {
        <div class="detail-container">
            <Suspense fallback=|| view! { <p class="loading">"Loading technique..."</p> }>
                <ErrorBoundary fallback=|_| view! { <p>"Server error occurred."</p> }>
                    
                    {move || match move_resource.get() {
                        Some(Ok(Some(m))) => view! {
                            <div class="move-card">
                                <div class="card-header">
                                    <div class="header-title">
                                        <h2>{m.name}</h2>
                                        <span class="category-badge">{m.category}</span>
                                    </div>
                                    <div class="difficulty-badge">
                                        "Level " {m.difficulty}
                                    </div>
                                </div>

                                <div class="card-body">
                                    {m.image_cover.map(|img| view! {
                                        <img src=img alt="Technique cover" class="move-image"/>
                                    })}

                                    <div class="notes-section">
                                        <h3>"Study Notes"</h3>
                                        {match m.notes {
                                            Some(text) => view! { <p>{text}</p> }.into_any(),
                                            None => view! { <p class="empty-text">"No notes recorded yet."</p> }.into_any(),
                                        }}
                                    </div>
                                    {m.video_link.map(|link| {
                                        let safe_embed_link = get_embed_url(&link);
                                        
                                        view! {
                                            <div class="video-container">
                                                <iframe 
                                                    src=safe_embed_link
                                                    title="Technique Video"
                                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                                    allowfullscreen
                                                ></iframe>
                                            </div>
                                        }
                                    })}
                                </div>
                                <div class="card-footer">
                                </div>
                            </div>
                        }.into_any(),
                        
                        Some(Ok(None)) => view! {
                            <div class="move-card empty-card">
                                <h2>"Technique Not Found"</h2>
                                <p>"It may have been deleted."</p>
                            </div>
                        }.into_any(),
                        
                        _ => ().into_any()
                    }}
                    
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
pub fn NoMoveSelected() -> impl IntoView {
    view! {
        <div class="detail-container">
            <div class="move-card empty-state-card">
                <div class="empty-state-content">
                    <svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    
                    <h2>"Select a Technique"</h2>
                    <p>"Choose a move from the library to view its details, study notes, and video references."</p>
                </div>
            </div>
        </div>
    }
}