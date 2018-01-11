extern crate rusqlite;

pub type Connection = rusqlite::Connection;

pub struct Database {
    pub connection: Connection,
}

pub fn open(db_file: &str) -> Database {
    let conn = Connection::open(db_file).unwrap();

    prepare_database(&conn);

    Database {
        connection: conn,
    }
}


fn prepare_database(conn: &Connection) -> bool {
    let latest_db_version = 1;
    let mut db_version: u32 = conn.query_row("PRAGMA user_version", &[], |row| row.get(0)).unwrap();

    eprintln!("Current DB version is {}, latest is {}", db_version, latest_db_version);

    if db_version == 0 {
        db_version = upgrade_db_from_0(conn);
    }
    if db_version == 1 {
        // db_version = upgrade_db_from_1(conn);
    }

    // Return state of DB upgrade
    db_version == latest_db_version
}


fn upgrade_db_from_0(conn: &Connection) -> u32 {
    let old_version = 0;
    let new_version = 1;
    eprintln!("Upgrading DB version from {} to {}", old_version, new_version);

    let stmt = "CREATE TABLE hosts (
        name         TEXT PRIMARY KEY,
        ip           TEXT NOT NULL,
        last_updated TEXT NOT NULL,
        last_touched TEXT NOT NULL
    )";

    match conn.execute(stmt, &[]) {
        Ok(_updated) => {
            conn.execute(&format!("PRAGMA user_version = {}", new_version), &[]).unwrap();
            new_version
        },
        Err(_err) => {
            eprintln!("DB upgrade failed");
            old_version
        },
    }
}