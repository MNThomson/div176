CREATE TABLE users (
    uid SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    phone_number TEXT NOT NULL CHECK (phone_number ~ '^[0-9]{10}$'),
    display_name TEXT NOT NULL,
    legal_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    -- stats
    last_login_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);

CREATE TABLE user_sessions (
    uid INTEGER NOT NULL REFERENCES users(uid),
    token TEXT NOT NULL UNIQUE,
    PRIMARY KEY(uid, token),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_user_sessions_user ON user_sessions(uid);

CREATE TABLE groups (
    gid SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    is_qualification BOOLEAN NOT NULL,
    qualification_expiration_yrs INTEGER,
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    CONSTRAINT check_qualification_expiration CHECK (
        is_qualification = TRUE OR
        (is_qualification = FALSE AND qualification_expiration_yrs IS NULL)
    )
);

CREATE TABLE users_groups (
    ug_id SERIAL PRIMARY KEY,
    uid INTEGER NOT NULL REFERENCES users(uid),
    gid INTEGER NOT NULL REFERENCES groups(gid),
    issued_at TIMESTAMPTZ,
    UNIQUE NULLS NOT DISTINCT (uid, gid, issued_at),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE event_types (
    etid SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    colour TEXT CHECK (colour ~ '^[0-9A-Fa-f]{6}$'),
    event_level_signup BOOL NOT NULL,
    hours_require_linked_signup BOOL NOT NULL,
    description TEXT,
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);
INSERT INTO event_types (etid, name, colour, event_level_signup, hours_require_linked_signup, description) VALUES 
(1, 'Duty', '00FF00', FALSE, TRUE, 'Volunteering at community events'),
(2, 'Training', 'FF0000', TRUE, TRUE, 'Training where you get a certification'),
(3, 'Meeting', 'FFFF00', TRUE, TRUE, 'Weekly meeting nights and sim club'),
(4, 'Admin', '0000FF', TRUE, TRUE, 'Other or misc administrative work');

CREATE TABLE events (
    eid SERIAL PRIMARY KEY,
    etid INTEGER NOT NULL REFERENCES event_types(etid),
    event_name TEXT NOT NULL,
    organizer_details TEXT,
    location TEXT,
    notes TEXT,
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);

CREATE TABLE shifts (
    sid SERIAL PRIMARY KEY,
    eid INTEGER NOT NULL REFERENCES events(eid),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    duty_lead INTEGER REFERENCES users(uid),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_hours (
    hid SERIAL PRIMARY KEY,
    uid INTEGER NOT NULL REFERENCES users(uid),
    etid INTEGER NOT NULL REFERENCES event_types(etid),
    sid INTEGER REFERENCES shifts(sid),
    date DATE NOT NULL,
    hours DECIMAL(5,2) NOT NULL CHECK (hours >= 0),
    description TEXT,
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    UNIQUE(uid, etid, date)
);

CREATE TABLE users_shifts (
    uid INTEGER NOT NULL REFERENCES users(uid),
    sid INTEGER NOT NULL REFERENCES shifts(sid),
    PRIMARY KEY (uid, sid),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMPTZ
);

CREATE TABLE vehicles (
    vid SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    notes TEXT,
    requires_group_to_drive INTEGER REFERENCES groups(gid),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);

CREATE TABLE shifts_vehicles (
    sid INTEGER NOT NULL REFERENCES shifts(sid),
    vid INTEGER NOT NULL REFERENCES vehicles(vid),
    PRIMARY KEY (sid, vid),
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_shifts_vehicles_sid ON shifts_vehicles(sid);

CREATE TABLE comments (
    cid SERIAL PRIMARY KEY,
    eid INTEGER NOT NULL REFERENCES events(eid),
    sid INTEGER REFERENCES shifts(sid),
    comment_text TEXT NOT NULL,
    -- stats
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);
