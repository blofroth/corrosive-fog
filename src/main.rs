#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;
extern crate corrosive_fog;

use rocket::{State};
use rocket_contrib::{JSON};
use corrosive_fog::model::*;
use corrosive_fog::persist::{NoteRepository, InMemNoteRepository};

// TODO: diesel
// https://mgattozzi.com/diesel-powered-rocket
// but with State<DB> instead

// TODO: nix? like pijul?

#[get("/")]
fn index() -> JSON<ApiRoot> {
    JSON(Default::default())
}

#[get("/<username>")]
fn user(username: &str) -> JSON<User> { 
    JSON(User {
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
fn get_user_notes(username: &str, note_repo: State<InMemNoteRepository>) -> Result<JSON<Notes>, String> {
    match note_repo.get_notes_for_user(username.into()) {
        Ok(notes) =>
            Ok(JSON(Notes {
                notes: notes,
                ..Default::default()
            })),
        Err(msg) => Err(msg)
    }
}

#[get("/<username>/notes/<guid>")]
fn get_user_note(username: &str, guid: &str) -> JSON<NoteWrapper> { 
    JSON(NoteWrapper {
        note: vec![Note {
            guid: guid.into(),
            ..Default::default()
        }]
    })
}

#[put("/<username>/notes", data = "<note_changes>")]
fn put_user_notes(username: &str, note_changes: JSON<NoteChanges>) { 
    println!("{:?}", note_changes);
}

fn main() {
    let mut note_repo = InMemNoteRepository::new();
    note_repo.add_user("sally".into());
    note_repo.upsert_note("sally".into(), example_note());

    rocket::ignite()
        .manage(note_repo)
        .mount("/api/1.0/", 
            routes![index, user, get_user_notes, get_user_note, put_user_notes])
        .launch();
}

fn example_note() -> Note { 
    Note {
        guid: "123".into(),
        ..Default::default()
    }
}