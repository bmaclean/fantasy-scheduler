use dialoguer::Select;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("{}", league_selection);

    Ok(())
}
