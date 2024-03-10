use std::fmt;

// let's rust compiler generate Debug trait for the struct
#[derive(Debug)]
struct Footballer {
    name: String,
    country: String,
    goals_scored: u32,
    matches_played: u32,
    trophies_won: Vec<String>, // To store a list of trophies
}

// uncomment below code to get the code compiling


// impl fmt::Display for Footballer {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} ({}) - Goals: {}, Matches: {}, Trophies: {}", 
//                self.name, self.country, self.goals_scored, self.matches_played, self.trophies_won.join(", "))
//     }
// }

fn main() {
    let player = Footballer {
        name: "Lionel Messi".to_string(),
        country: "Argentina".to_string(),
        goals_scored: 796, // Updated goal count
        matches_played: 1073, // Updated match count
        trophies_won: vec!["Champions League".to_string(), "La Liga".to_string(), "Copa del Rey".to_string(), "World Cup".to_string()],
    };

    // this will not work as Rust does not know how to Display a struct
    // developer should implement Display trait
    println!("{}", player);  
    println!("{:?}", player); 
}