--- Song info
CREATE TABLE IF NOT EXISTS songs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT DEFAULT 'UNKNOWN TITLE',
    artist TEXT DEFAULT 'UNKNOWN ARTISt',
    album TEXT DEFAULT 'UNKNOWN ALBUM',
    year INTEGER DEFAULT 0,
    genre TEXT DEFAULT 'UNKNOWN GENRE',
    track_number INTEGER DEFAULT 0,
    total_tracks INTEGER DEFAULT 0,
    disc_number INTEGER DEFAULT 0,
    total_discs INTEGER DEFAULT 0,
    album_artist TEXT DEFAULT 'UNKNOWN',
    file_path TEXT NOT NULL UNIQUE,
    has_cover BOOLEAN DEFAULT 0,
    origin_readable DEFAULT 'UNKNOWN ORIGIN',
    origin TEXT DEFAULT 'UNKNOWN ORIGIN',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_songs_artist ON songs(artist);
CREATE INDEX IF NOT EXISTS idx_songs_album ON songs(album);

-- Create the playlists table
CREATE TABLE IF NOT EXISTS playlists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create the junction table for songs
CREATE TABLE IF NOT EXISTS playlist_songs (
    playlist_id INTEGER NOT NULL,
    song_path TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    PRIMARY KEY (playlist_id, song_path),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
);
