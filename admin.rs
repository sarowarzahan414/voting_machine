use rusqlite::Connection;

pub fn authenticate_admin() -> bool {
    let password = rpassword::prompt_password("Enter admin password: ").unwrap();
    password == "admin123"
}

pub fn admin_menu(conn: &Connection) {
    println!("Admin Menu");
    println!("1. Create Ballot");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    
    if choice.trim() == "1" {
        create_ballot(conn);
    }
}

fn create_ballot(conn: &Connection) {
    println!("Creating a new ballot...");

    let mut stmt = conn.prepare("INSERT INTO offices (name) VALUES (?)").unwrap();
    for _ in 0..3 {
        println!("Enter office name: ");
        let mut office_name = String::new();
        std::io::stdin().read_line(&mut office_name).unwrap();
        stmt.execute([office_name.trim()]).unwrap();
        
        let office_id = conn.last_insert_rowid();
        println!("Enter candidates for this office (name,party), one per line. Type 'done' when finished.");
        
        let mut candidate_stmt = conn.prepare("INSERT INTO candidates (name, party, office_id) VALUES (?, ?, ?)").unwrap();
        loop {
            let mut candidate = String::new();
            std::io::stdin().read_line(&mut candidate).unwrap();
            
            if candidate.trim() == "done" {
                break;
            }
            
            let parts: Vec<&str> = candidate.trim().split(',').collect();
            if parts.len() != 2 {
                println!("Invalid format. Enter 'name,party'");
                continue;
            }
            candidate_stmt.execute([parts[0], parts[1], &office_id.to_string()]).unwrap();
        }
    }
}

