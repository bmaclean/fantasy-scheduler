use dialoguer::{Select, Input};
use std::process;
use serde::Deserialize;
use reqwest::ClientBuilder;

#[derive(Deserialize, Debug)]
struct LeagueSettings {
    num_teams: u32,
    playoff_teams: u32,
    playoff_week_start: u32,
    start_week: u32,
    playoff_round_type: u32,
    divisions: u32
}

#[derive(Deserialize, Debug)]
struct LeagueMetadata {
    division1: Option<String>,
    division2: Option<String>
}

#[derive(Deserialize, Debug)]
struct LeagueData {
    name: String,
    total_rosters: u32,
    season_type: String,
    settings: LeagueSettings,
    metadata: LeagueMetadata
}

#[derive(Deserialize, Debug)]
struct RosterSettings {
    division: u32
}

#[derive(Deserialize, Debug)]
struct Roster {
    owner_id: String,
    settings: RosterSettings,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Time to build your fantasy schedule. We need some information to get you started.");

    let league_options = vec!["Sleeper", "NFL.com", "ESPN"];
    let league_index = Select::new().with_prompt("Select your fantasy platform")
        .items(&league_options)
        .default(0)
        .interact()?;
    let league_selection = league_options[league_index];

    if league_selection != "Sleeper" {
        println!("This CLI currently only supports Sleeper leagues.");
        process::exit(1);
    }

    // fetch_league_data
    let league_id : String = Input::new()
    .with_prompt("What is your league ID?")
    .interact_text()?;
    let league_url = format!("https://api.sleeper.app/v1/league/{}", league_id);

    println!("Fetching your Sleeper league data using URL: {}", league_url);
    
    let client = ClientBuilder::new().build()?;
    let league_response = client.get(&league_url).send().await?;

    if !league_response.status().is_success() {
        println!("There was a problem fetching your league data. Please double-check the provided ID.");
        process::exit(1);
    }

    println!("League data retrieved successfully.");
    let league_data: LeagueData = league_response.json().await?;
    // println!("{:?}", league_data);

    // fetch_roster_data
    let league_rosters_url = format!("{}/rosters", league_url);
    let rosters_response = client.get(&league_rosters_url).send().await?;

    if !rosters_response.status().is_success() {
        println!("There was a problem fetching your league roster data.");
        process::exit(1);
    }

    println!("Roster data retrieved successfully.");
    let roster_data: Vec<Roster> = rosters_response.json().await?;
    // println!("{:?}", roster_data);

    println!("We'll now need some input on how the schedule should be configured.");

    let schedule_options = vec!["Round Robin", "2x Divisional Team Matchups"];
    let schedule_option_index = Select::new().with_prompt("Choose from one of the following schedule options")
        .items(&schedule_options)
        .default(0)
        .interact()?;
    let schedule_option_selection = schedule_options[schedule_option_index];

    let yes_no_options = vec!["Yes", "No"];
    let rivalry_week_index = Select::new().with_prompt("Should your schedule include a Rivalry Week?")
        .items(&yes_no_options)
        .default(0)
        .interact()?;
    let rivalry_week_toggle = yes_no_options[rivalry_week_index];

    let rivalry_week_input: String;
    if rivalry_week_toggle == "Yes" {
        rivalry_week_input = Input::new()
        .with_prompt("What week should rivalry week take place?")
        .interact_text()?;

        let rivalry_week_number: u32 = rivalry_week_input.parse::<u32>().unwrap();

        if rivalry_week_number > 17 {
            println!("Invalid roster week number was provided.");
            process::exit(1);
        }
    }
    Ok(())
}
