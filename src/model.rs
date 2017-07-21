#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub user_name: String,
    pub first_name: String, 
    pub last_name: String,
    pub notes_ref: ApiRef,
    pub lastest_sync_revision: i32,
    pub current_sync_guid: String
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Note { 
    pub guid: String,
    #[serde(rename = "ref")]
    pub note_ref: Option<String>,
    pub title: String,
    pub note_content: String,
    pub note_content_version: f64,
    // TODO: date with custom serializer?
    pub last_change_date: String,
    pub last_metadata_change_date: String,
    pub create_date: String,
    pub last_sync_revision: String,
    pub open_on_startup: bool,
    pub pinned: bool,
    pub tags: Vec<String>
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
pub struct ApiRoot {
    #[serde(rename = "user-ref")]
    pub user_ref: ApiRef,
    pub oauth_request_token_url: String,
    pub oauth_authorize_url: String,
    pub oauth_access_token_url: String,
    #[serde(rename = "api-version")]
    pub api_version: String
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ApiRef {
    pub api_ref: String,
    pub href: String, 
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Notes { 
    pub lastest_sync_revision: i32,
    pub notes: Vec<Note>
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct NoteChanges { 
    pub lastest_sync_revision: i32,
    pub note_changes: Vec<Note>
}

#[derive(Serialize,Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct NoteWrapper { 
    pub note: Vec<Note>
}