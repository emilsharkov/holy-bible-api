CREATE TABLE bibles (
    bible_id SERIAL PRIMARY KEY,
    language VARCHAR(100) NOT NULL,
    version VARCHAR(100)
);

CREATE TABLE verses (
    bible_id INT NOT NULL,
    book INT NOT NULL,
    chapter INT NOT NULL,
    verse INT NOT NULL,
    text TEXT NOT NULL,
    FOREIGN KEY (bible_id) REFERENCES bibles(bible_id)
);

CREATE INDEX idx_verses_bible_id ON verses(bible_id);

CREATE TABLE audio_bibles (
    audio_bible_id SERIAL PRIMARY KEY,
    language VARCHAR(100) NOT NULL,
    version VARCHAR(100)
);

CREATE TABLE audio_bible_chapters (
    audio_bible_id INT NOT NULL,
    book INT NOT NULL,
    chapter INT NOT NULL,
    FOREIGN KEY (audio_bible_id) REFERENCES audio_bibles(audio_bible_id)
);