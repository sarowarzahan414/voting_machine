mod admin;
mod election;
mod voter;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open("election.db").unwrap();
    
    loop {
        println!("1. Admin Login");
        println!("2. Register Voter");
        println!("3. Cast Vote");
        println!("4. Open Election");
        println!("5. Close Election");
        println!("6. Tally Votes");
        println!("7. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                if admin::authenticate_admin() {
                    println!("Welcome, Admin.");
                    admin::admin_menu(&conn);
                } else {
                    println!("Invalid password.");
                }
            }
            "2" => voter::register_voter(&conn),
            "3" => voter::cast_vote(&conn),
            "4" => election::open_election(&conn),  // Calling open_election
            "5" => election::close_election(&conn), // Calling close_election
            "6" => election::tally_votes(&conn),
            "7" => break,
            _ => println!("Invalid option. Try again."),
        }
    }
}

