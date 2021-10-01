use crate::{Roster, RosterSettings};

pub struct Matchup {
    // TODO: Team
    team1: String,
    team2: String
}

pub struct ScheduleOptions {
    // TODO: ScheduleType
    pub schedule_type: String,
    pub num_weeks: u32,
    pub rosters: Vec<Roster>,
    pub divisions: Vec<u32>
}

struct ScheduleError {

}

pub fn generate_schedule(opts: ScheduleOptions) -> Vec<Vec<Matchup>> {
    return match opts.schedule_type.as_str() {
        "Round Robin" => generate_random_schedule(opts),
        "2x Divisional Team Matchups" => generate_divisional_schedule(opts),
        _ => vec![]
    }
}

fn generate_random_schedule(opts: ScheduleOptions) -> Vec<Vec<Matchup>> {
    // TODO:
    let schedule: Vec<Vec<Matchup>> = vec![];

    schedule
}

fn generate_divisional_schedule(opts: ScheduleOptions) -> Vec<Vec<Matchup>> {
    let schedule: Vec<Vec<Matchup>> = vec![];

    let num_division_matchups = opts.rosters.len() / opts.divisions.len();
    let teams_by_division = opts.rosters.as_chunks();
    // generate hash for team: opps remaining

    // first division matchups
    for n in 1..num_division_matchups {

    }

    schedule
}
