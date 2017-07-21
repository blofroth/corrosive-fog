use persist::NoteRepository;
use model::Note;

type SyncError = String;

pub struct SyncController<'a, T : NoteRepository + Send + Sync + 'a> {
    repo: &'a T
}

impl<'a, T: NoteRepository + Send + Sync + 'a> SyncController<'a, T> {

    pub fn new(repo: &'a T) -> Self {
        SyncController { repo: repo }
    }

    pub fn get_notes_for_user(&self, username: String) -> Result<Vec<Note>, SyncError> {
       let notes = self.repo.get_notes_for_user(username)?;

       Ok(notes.into_iter().map(|n| n.into()).collect())
    }

    pub fn get_note(&self, username: String, guid: String) -> Result<Note, SyncError> {
        let note = self.repo.get_note(username, guid)?;

        Ok(note.into())
    }
    
    pub fn upsert_note(&self, username: String, note: Note) -> Result<bool, SyncError> {
        self.repo.upsert_note(username, note.into()) 
    }

}