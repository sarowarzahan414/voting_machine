use rusqlite::Connection;

pub fn open_election(conn: &Connection) {
    conn.execute("INSERT INTO election (is_open) VALUES (1)", []).unwrap();
    println!("Election is now open!");
}

pub fn close_election(conn: &Connection) {
    conn.execute("UPDATE election SET is_open = 0", []).unwrap();
    println!("Election is now closed!");
}

pub fn tally_votes(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name FROM offices").unwrap();
    let offices = stmt.query_map([], |row| {
    Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
}).unwrap();
    
    for office in offices {
        let (office_id, office_name) = office.unwrap();
        println!("Results for {}:", office_name);
        
        let mut vote_stmt = conn.prepare("
            SELECT c.name, COUNT(v.id) 
            FROM votes v 
            JOIN candidates c ON v.candidate_id = c.id 
            WHERE v.office_id = ? 
            GROUP BY c.name").unwrap();
        
        let vote_counts = vote_stmt.query_map([office_id], |row| {
    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
}).unwrap();
        
        for vote in vote_counts {
            let (candidate_name, count) = vote.unwrap();
            println!("{}: {} votes", candidate_name, count);
        }
    }
}

