#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate corrosive_fog;

use rocket::{Rocket, State};
use rocket_contrib::{Json};
use std::sync::{RwLock};
use corrosive_fog::model::*;
use corrosive_fog::persist::{NoteRepository, InMemNoteRepository};


// TODO: move into inmem implementation?
type SyncedRepo = RwLock<InMemNoteRepository>;

#[get("/")]
fn index() -> Json<ApiRoot> {
    Json(Default::default())
}

#[get("/<username>")]
fn user(username: String) -> Json<User> { 
    Json(User {
        user_name : username.into(),
        first_name : "sally".into(),
        last_name : "walters".into(),
        notes_ref : ApiRef { 
            api_ref : "http://domain/api/1.0/sally/notes".into(),
            href : "http://domain/sally/notes".into()
        },
        lastest_sync_revision: 456,
        current_sync_guid: "ff2e91b2-1234-4eab-3000-abcde49a7705".into()
    })
}

#[get("/<username>/notes")]
fn get_user_notes(username: String, note_repo: State<SyncedRepo>) -> Result<Json<Notes>, String> {
    let locked_repo = note_repo.read().unwrap();
    match locked_repo.get_notes_for_user(username.into()) {
        Ok(notes) =>
            Ok(Json(Notes {
                notes: notes,
                ..Default::default()
            })),
        Err(msg) => Err(msg)
    }
}

#[get("/<username>/notes/<guid>")]
fn get_user_note(username: String, guid: String, note_repo: State<SyncedRepo>) -> Result<Json<NoteWrapper>, String> {
    let locked_repo = note_repo.read().unwrap();
    match locked_repo.get_note(username.into(), guid.into()) {
        Ok(note) => Ok(Json(NoteWrapper {
            note: vec![note]
        })),
        Err(msg) => Err(msg)
    }
}

#[put("/<username>/notes", data = "<note_changes>")]
fn put_user_notes(username: String, note_changes: Json<NoteChanges>,
        note_repo: State<SyncedRepo>) -> Result<(),String> {
    let mut locked_repo = note_repo.write().unwrap();

    let notes = note_changes.into_inner().note_changes.into_iter();
    for note in notes {
        locked_repo.upsert_note(username.clone(), note)
            .map(|_| ())?
    }

    Ok(())
}

fn main() {
   mount_routes().launch();
}

fn mount_routes() -> Rocket {
    let mut note_repo = InMemNoteRepository::new();
    note_repo.add_user("sally".into());
    let synced_repo: SyncedRepo = RwLock::new(note_repo);

    rocket::ignite()
        .manage(synced_repo)
        .mount("/api/1.0/", 
            routes![index, user, get_user_notes, get_user_note, put_user_notes])
}

#[cfg(test)]
mod test {
    use super::mount_routes;
    use rocket::http::{Status, Method};
    use rocket::http::ContentType;
    use rocket::local::Client;

    #[test]
    fn test_api_root() {
        let rocket = mount_routes();
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/api/1.0/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_user() {
        let rocket = mount_routes();

        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.get("/api/1.0/sally").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_user_notes() {
        let rocket = mount_routes();

        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.get("/api/1.0/sally/notes").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    const UPDATE_TEMPLATE: &str = r#"{
            "lastest-sync-revision": 0,
            "note-changes": [
                {
                    "guid": "{NOTE_GUID}",
                    "ref": null,
                    "title": "",
                    "note-content": "",
                    "note-content-version": 0,
                    "last-change-date": "",
                    "last-metadata-change-date": "",
                    "create-date": "",
                    "last-sync-revision": "",
                    "open-on-startup": false,
                    "pinned": false,
                    "tags": []
                }
            ]
        }"#;

    #[test]
    fn test_put_user_notes() {
        let rocket = mount_routes();

        let client = Client::new(rocket).expect("valid rocket instance");

        let note1 = UPDATE_TEMPLATE.replace("NOTE_GUID", "123");
        let note2 = UPDATE_TEMPLATE.replace("NOTE_GUID", "124");

        let response = client.put("/api/1.0/sally/notes")
            .header(ContentType::JSON)
            .body(note1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client.put("/api/1.0/sally/notes")
            .header(ContentType::JSON)
            .body(note2)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let mut response = client.get("/api/1.0/sally/notes")
            .dispatch();
        let body_str = response.body_string().expect("some body");
        
        assert!(body_str.contains("123"));
        assert!(body_str.contains("124"));
    }
}