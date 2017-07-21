CREATE TABLE notes (
  guid VARCHAR PRIMARY KEY,
  title VARCHAR NOT NULL,
  note_content TEXT NOT NULL,
  note_content_version double precision NOT NULL,
  last_change_date VARCHAR NOT NULL,
  last_metadata_change_date VARCHAR NOT NULL,
  last_sync_revision VARCHAR NOT NULL,
  create_date VARCHAR NOT NULL,
  open_on_startup BOOLEAN NOT NULL,
  pinned BOOLEAN NOT NULL
)
