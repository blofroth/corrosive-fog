use model;
use super::schema::notes;

#[derive(Debug,Clone,Queryable,Insertable)]
#[table_name="notes"]
pub struct Note { 
    pub guid: String,
    pub title: String,
    pub note_content: String,
    pub note_content_version: f64,
    pub last_change_date: String,
    pub last_metadata_change_date: String,
    pub create_date: String,
    pub last_sync_revision: String,
    pub open_on_startup: bool,
    pub pinned: bool,
}

impl From<model::Note> for Note {
    fn from(note: model::Note) -> Self {
        Note {
            guid: note.guid,
            title: note.title,
            note_content: note.note_content,
            note_content_version: note.note_content_version,
            last_change_date: note.last_change_date,
            last_metadata_change_date: note.last_metadata_change_date,
            create_date: note.create_date,
            last_sync_revision: note.last_sync_revision,
            open_on_startup: note.open_on_startup,
            pinned: note.pinned
        }
    }
}

impl From<Note> for model::Note {
    fn from(note: Note) -> model::Note {
        model::Note {
            guid: note.guid,
            note_ref: None,
            title: note.title,
            note_content: note.note_content,
            note_content_version: note.note_content_version,
            last_change_date: note.last_change_date,
            last_metadata_change_date: note.last_metadata_change_date,
            create_date: note.create_date,
            last_sync_revision: note.last_sync_revision,
            open_on_startup: note.open_on_startup,
            pinned: note.pinned,
            tags: Vec::new()
        }
    }
}