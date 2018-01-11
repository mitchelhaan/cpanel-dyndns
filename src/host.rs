extern crate chrono;
extern crate curl;

use cpanel;
use database::Database;

#[derive(Default, Debug)]
pub struct Host {
    pub name: String,
    pub ip: String,
    pub last_updated: String,
    pub last_touched: String,
}


pub fn read(db: &Database, hostname: &str) -> Option<Host> {
    match db.connection.query_row("SELECT name, ip, last_updated, last_touched FROM hosts WHERE name = ?1", &[&hostname], |row| {
        Host {
            name: row.get(0),
            ip: row.get(1),
            last_updated: row.get(2),
            last_touched: row.get(3),
        }
    }) {
        Ok(_result) => {
            Some(_result)
        },
        Err(_err) => {
            eprintln!("Failed to read host: {}", _err);
            None
        },
    }
}

pub fn update(db: &Database, hostname: &str, ip: &str) -> Option<Host> {
    let existing_host = match read(db, hostname) {
        Some(val) => val,
        None => Default::default(),
    };

    // Host wasn't found, create it
    if existing_host.name == "" {
        match db.connection.execute("INSERT INTO hosts (name, ip, last_updated, last_touched) VALUES (?1, ?2, datetime('now'), datetime('now'))", &[&hostname, &ip]) {
            Ok(_updated) => {},
            Err(_err) => { eprintln!("Failed to create host: {}", _err); },
        }
    } else {
        // Host exists, but IP is incorrect
        if existing_host.ip != ip {
            // Update DNS entry
            cpanel::update_dns_entry(hostname, ip);

            // Update database entry
            match db.connection.execute("UPDATE hosts SET ip = ?2, last_updated = datetime('now'), last_touched = datetime('now') WHERE name = ?1", &[&hostname, &ip]) {
                Ok(_updated) => {},
                Err(_err) => { eprintln!("Failed to update host: {}", _err); },
            }
        } else {
            match db.connection.execute("UPDATE hosts SET last_touched = datetime('now') WHERE name = ?1", &[&hostname]) {
                Ok(_updated) => {},
                Err(_err) => { eprintln!("Failed to touch host: {}", _err); },
            }
        }
    }

    read(db, hostname)
}
