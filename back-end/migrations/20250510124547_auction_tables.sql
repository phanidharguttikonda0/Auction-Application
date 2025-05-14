-- users table
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       mail_id VARCHAR(250) NOT NULL UNIQUE,
                       username VARCHAR(18) NOT NULL UNIQUE,
                       password VARCHAR(64) NOT NULL,
                       DOB DATE NOT NULL
);

-- rooms table
CREATE TABLE rooms (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       max_participants INT NOT NULL,
                       owner_id INT REFERENCES users(id),
                       accessibility VARCHAR(10) CHECK (accessibility IN ('PUBLIC', 'PRIVATE')),
                       room_status VARCHAR(20) CHECK (room_status IN ('WAITING', 'ONGOING', 'COMPLETED')),
                       createdAt TIMESTAMPTZ DEFAULT now()
);

-- Partial Unique Index for owner_id with active room_status
CREATE UNIQUE INDEX unique_owner_active_room
    ON rooms (owner_id)
    WHERE room_status IN ('WAITING', 'ONGOING');

-- participants table
CREATE TABLE participants (
                              id SERIAL PRIMARY KEY,
                              room_id UUID REFERENCES rooms(id),
                              participant_id INT REFERENCES users(id),
                              team_selected TEXT CHECK (team_selected IN (
                                                                          'MUMBAIINDIANS','CHENNAISUPERKINGS','SUNRISERSHYDERABAD',
                                                                          'ROYALCHALLENGERSBENGALURU','KOLKATAKINGKNIGHTRIDERS','PUNJABKINGS',
                                                                          'RAJASTANROYALS','GUJARATTITANS','DELHICAPITALS','LUCKNOWSUPERGAINS'
                                  )),
                              CONSTRAINT unique_team_per_room UNIQUE (room_id, team_selected)
);

-- stats table
CREATE TABLE stats (
                       id SERIAL PRIMARY KEY,
                       matches INT NOT NULL,
                       runs INT,
                       average FLOAT,
                       strike_rate FLOAT,
                       highest_score INT,
                       fifties INT,
                       hundreads INT,
                       wickets INT,
                       three_wickets INT,
                       five_wickets INT,
                       stats_from TEXT CHECK (stats_from IN ('List-A', 'IPL', 'T20s'))
);

-- players table
CREATE TABLE players (
                         id SERIAL PRIMARY KEY,
                         name VARCHAR(120) NOT NULL,
                         DOB DATE NOT NULL,
                         role TEXT CHECK (role IN ('BAT', 'BOWL', 'AR')) NOT NULL,
                         base_price INT NOT NULL,
                         stats_id INT REFERENCES stats(id),
                         country VARCHAR(25) NOT NULL
);

-- sold_players table
CREATE TABLE sold_players (
                              player_id INT REFERENCES players(id),
                              room_id UUID REFERENCES rooms(id),
                              amount INT NOT NULL,
                              participant_id INT REFERENCES participants(id)
);

-- unsold_players table
CREATE TABLE unsold_players (
                                player_id INT REFERENCES players(id),
                                room_id UUID REFERENCES rooms(id)
);

-- Additional Indexes for query performance
CREATE INDEX idx_participants_room_id ON participants(room_id);
CREATE INDEX idx_participants_participant_id ON participants(participant_id);
CREATE INDEX idx_players_country ON players(country);
CREATE INDEX idx_players_role ON players(role);
CREATE INDEX idx_stats_from ON stats(stats_from);
CREATE INDEX idx_sold_players_room_id ON sold_players(room_id);
CREATE INDEX idx_unsold_players_room_id ON unsold_players(room_id);
CREATE INDEX idx_user_id ON users(id) ;
CREATE INDEX idx_user_username ON users(username);
