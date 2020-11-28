use clap::{App, Arg, SubCommand};
use chrono::Datelike;
use std::io::Write;

pub fn run_clap(year: i32, last_day: &str, f: impl FnOnce(Option<&str>)) {
    let matches = App::new("My Super Program")
        .author("Giacomo Stevanato <giaco.stevanato@gmail.com>")
        .about(format!("My solutions to Advent of code {}", year).as_str())
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("Run the solution for the day $DAY")
            .default_value(last_day)
            .takes_value(true))
        .subcommand(SubCommand::with_name("session")
            .about("Sets the session token to use")
            .arg(Arg::with_name("SESSION")
                .required(true)))
        .subcommand(SubCommand::with_name("input")
            .about("Download an input file")
            .arg(Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Download the input file for the day $DAY")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("setup")
            .about("Setup the template file for the day $DAY")
            .arg(Arg::with_name("DAY")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("session", Some(session_args)) => set_session(session_args.value_of("SESSION").unwrap_or("")),
        ("input", Some(input_args)) => {
            match input_args.value_of("day") {
                Some("all") => download_all_inputs(year),
                Some(day) => {
                    let parsed_day = day.parse::<u32>().expect("Invalid parameter");
                    assert!(1 <= parsed_day && parsed_day <= 25, "Invalid parameter");
                    get_input(year, day);
                },
                None => {
                    let today = chrono::offset::Local::today();
                    if today.year() == year && today.month() == 12 {
                        get_input(year, &format!("{}", today.day()));
                    } else {
                        download_all_inputs(year);
                    };
                }
            }
        },
        ("setup", Some(setup_args)) => {
            let day = setup_args.value_of("DAY").expect("Expected parameter");
            let parsed_day = day.parse::<u32>().expect("Invalid parameter");
            assert!(1 <= parsed_day && parsed_day <= 25, "Invalid parameter");

            static TEMPLATE: &str = include_str!("../template.rs");
            let mut day_file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(format!("src/day{}.rs", day))
                .expect("Failed to create template file");
            write!(day_file, "{}", TEMPLATE).expect("Failed to write to template file");
        }
        _ => f(matches.value_of("day")),
    }
}

pub fn get_input(year: i32, day: &str) -> String {
    if let Ok(input) = std::fs::read_to_string(format!("./input/{}/day{}.txt", year, day)) {
        return input;
    }

    let agent = create_agent(get_session());
    let input = download_input(&agent, year, day);

    input
}

fn session_file() -> std::path::PathBuf {
    directories::ProjectDirs::from("com.github", "Giuschi",  "Aoc-Session")
        .expect("Couldn't find a valid home directory")
        .config_dir()
        .to_path_buf()
}

fn set_session(session: &str) {
    let session_file = session_file();
    if let Some(parent) = session_file.parent() {
        std::fs::create_dir_all(parent).expect("Couldn't create parent directories");
    }
    let mut session_file = std::fs::File::create(&session_file).expect("Couldn't create config file");
    write!(session_file, "{}", session).expect("Couldn't write to config file");
}

fn get_session() -> String {
    let session_file = session_file();
    std::fs::read_to_string(&session_file)
        .expect("Couldn't open config file. Did you set it up properly?")
}

fn create_agent(session: String) -> ureq::Agent {
    let agent = ureq::agent();
    let mut session_cookie = ureq::Cookie::new("session", session);
    session_cookie.set_domain("adventofcode.com");
    agent.set_cookie(session_cookie);
    agent
}

fn download_input(agent: &ureq::Agent, year: i32, day: &str) -> String {
    print!("     - Downloading input for day {:<2}... ", day);

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let body = agent.get(&url)
        .call()
        .into_string()
        .expect("Input request failed");

    if body == "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" {
        panic!("Invalid session cookie");
    }

    let destination: std::path::PathBuf = format!("./input/{}/day{}.txt", year, day).into();
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).expect("Couldn't create parent directories");
    }
    let mut file = std::fs::File::create(&destination)
        .expect("Couldn't create input file");
    write!(file, "{}", body).expect("Couldn't write input file");

    println!("Input downloaded");

    body
}

fn download_all_inputs(year: i32) {
    let agent = create_agent(get_session());

    let today = chrono::offset::Local::today();
    let max_day = if today.year() == year {
        if today.month() == 12 {
            std::cmp::min(today.day(), 25)
        } else {
            println!("AdventOfCode {} hasn't started yet!", year);
            return;
        }
    } else {
        25
    };

    for day in 1..=max_day {
        let day = format!("{}", day);
        print!("Checking input for day {:<2} year {}.", day, year);

        if let Ok(_) = std::fs::File::open(format!("./input/{}/day{}.txt", year, day)) {
            println!("     - Input already downloaded.");
        } else {
            download_input(&agent, year, &day);
        }
    }
}
