use rusqlite::Connection;

pub fn register_voter(conn: &Connection) {
    println!("Registering a new voter...");

    let mut name = String::new();
    let mut dob = String::new();

    println!("Enter voter's name: ");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Enter voter's date of birth (YYYY-MM-DD): ");
    std::io::stdin().read_line(&mut dob).unwrap();

    conn.execute("INSERT INTO voters (name, dob) VALUES (?, ?)", &[name.trim(), dob.trim()]).unwrap();
    println!("Voter registered successfully.");
}


pub fn cast_vote(conn: &Connection) {
    println!("Enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    
    let voter_id = conn.query_row("SELECT id FROM voters WHERE name = ?", &[name.trim()], |row| row.get::<_, i64>(0));
    
    match voter_id {
        Ok(voter_id) => {
            let election_open: bool = conn.query_row("SELECT is_open FROM election", [], |row| row.get(0)).unwrap();
            if !election_open {
                println!("The election is closed.");
                return;
            }
            
            let mut stmt = conn.prepare("SELECT id, name FROM offices").unwrap();
            let office_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get::<_, String>(1)?))).unwrap();

            for office in office_iter {
                let (office_id, office_name) = office.unwrap();
                println!("Vote for {}:", office_name);
                
                let mut candidate_stmt = conn.prepare("SELECT id, name, party FROM candidates WHERE office_id = ?").unwrap();
                let candidates = candidate_stmt.query_map([office_id], |row| {
                    let id: i64 = row.get(0)?;
                    let name: String = row.get(1)?;
                    let party: Option<String> = row.get(2)?;
                    Ok((id, name, party))
                }).unwrap();
                
                for candidate in candidates {
                    let (id, name, party) = candidate.unwrap();
                    let party_display = party.as_deref().unwrap_or("No party"); // Handle Option
                    println!("{}: {} ({})", id, name, party_display);
                }
                
                println!("Enter candidate ID:");
                let mut candidate_choice = String::new();
                std::io::stdin().read_line(&mut candidate_choice).unwrap();
                
                let candidate_id: i64 = candidate_choice.trim().parse().expect("Invalid candidate ID");

                conn.execute("INSERT INTO votes (voter_id, candidate_id, office_id) VALUES (?, ?, ?)", [voter_id, candidate_id, office_id]).unwrap();
            }

            conn.execute("UPDATE voters SET has_voted = 1 WHERE id = ?", [voter_id]).unwrap();
            println!("Thank you for voting!");
        }
        Err(_) => {
            println!("Voter not registered.");
        }
    } // Closing match block
} // Closing function block

