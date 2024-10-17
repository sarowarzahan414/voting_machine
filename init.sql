CREATE TABLE voters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    dob TEXT NOT NULL,
    has_voted INTEGER DEFAULT 0
);

CREATE TABLE offices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE candidates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    party TEXT NOT NULL,
    office_id INTEGER,
    FOREIGN KEY (office_id) REFERENCES offices(id)
);

CREATE TABLE votes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voter_id INTEGER,
    candidate_id INTEGER,
    office_id INTEGER,
    FOREIGN KEY (voter_id) REFERENCES voters(id),
    FOREIGN KEY (candidate_id) REFERENCES candidates(id),
    FOREIGN KEY (office_id) REFERENCES offices(id)
);

CREATE TABLE election (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    is_open INTEGER DEFAULT 0
);

