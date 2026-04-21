--- Song info
CREATE TABLE IF NOT EXISTS songs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT DEFAULT 'Unknown Title',
    artist TEXT DEFAULT 'Unknown Artist',
    album TEXT DEFAULT 'Unknown Album',
    year INTEGER,
    genre TEXT,
    track_number INTEGER,
    total_tracks INTEGER,
    disc_number INTEGER,
    total_discs INTEGER,
    album_artist TEXT,
    file_path TEXT NOT NULL UNIQUE,
    has_cover BOOLEAN DEFAULT 0,
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
