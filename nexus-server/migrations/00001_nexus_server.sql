-- Create Replicants table
CREATE TABLE Replicants (
    replicant_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT,
    host TEXT,
    port INTEGER
);

-- Create Instances table
CREATE TABLE Instances (
    instance_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    replicant_id TEXT NOT NULL,
    version TEXT,
    host TEXT,
    port INTEGER,
    FOREIGN KEY (replicant_id) REFERENCES Replicants (replicant_id)
);

-- Create Tags table
CREATE TABLE Tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_id TEXT NOT NULL,
    type TEXT CHECK( type IN ('Replicant', 'Instance') ) NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY (entity_id) REFERENCES Replicants (replicant_id) ON DELETE CASCADE,
    FOREIGN KEY (entity_id) REFERENCES Instances (instance_id) ON DELETE CASCADE
);
