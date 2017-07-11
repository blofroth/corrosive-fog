use model::*;
use std::collections::HashMap;
use std::mem;

pub type PersistenceError = String;

pub trait NoteRepository {
    fn get_notes_for_user(&self, username: String) -> Result<Vec<Note>, PersistenceError>;
    fn get_note(&self, username: String, guid: String) -> Result<Note, PersistenceError>;
    fn upsert_note(&mut self, username: String, note: Note) -> Result<bool, PersistenceError>;
}

pub struct InMemNoteRepository {
    user_notes: HashMap<String, Vec<Note>>
}

impl InMemNoteRepository {
    pub fn new() -> InMemNoteRepository {
        InMemNoteRepository { user_notes: HashMap::new() }
    }

    pub fn add_user(&mut self, username: String) {
        self.user_notes.insert(username, Vec::new());
    }
} 

impl NoteRepository for InMemNoteRepository {
    fn get_notes_for_user(&self, username: String) -> Result<Vec<Note>, PersistenceError> {
        self.user_notes.get(&username)
            .map(|notes| notes.clone())
            .ok_or("Could not find username".into())
    }

    fn get_note(&self, username: String, guid: String) -> Result<Note, PersistenceError> {

        if let Some(notes) = self.user_notes.get(&username) {
            notes.iter()
                .filter(|note| note.guid == guid)
                .nth(0)
                .map(|note| note.clone())
                .ok_or("No note found for username and guid".into())
        } else {
            Err("Could not find username".into())
        }
    }

    fn upsert_note(&mut self, username: String, note: Note) -> Result<bool, PersistenceError> {
        let mut note = note;

        let notes = self.user_notes.get_mut(&username)
            .ok_or("Could not find username")?;

        {
            let maybe_found = notes.iter_mut()
                .filter(|found| found.guid == note.guid)
                .nth(0);

            if let Some(mut found_note) = maybe_found {
                mem::swap(&mut note, &mut found_note);
                return Ok(true)
            }
        }

        notes.push(note);
        Ok(false)
    }
}