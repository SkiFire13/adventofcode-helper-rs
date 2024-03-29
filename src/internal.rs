use chrono::Datelike;
use clap::{App, Arg, SubCommand};
use std::io::Write;

pub fn run_clap(year: i32, last_day: Option<&str>, src_dir: &str, f: impl FnOnce(Option<&str>)) {
    let matches = App::new("My Super Program")
        .author("Giacomo Stevanato <giaco.stevanato@gmail.com>")
        .about(format!("My solutions to Advent of code {}", year).as_str())
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Run the solution for the day $DAY")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("session")
                .about("Sets the session token to use")
                .arg(Arg::with_name("SESSION").required(true)),
        )
        .subcommand(
            SubCommand::with_name("input")
                .about("Download an input file")
                .arg(
                    Arg::with_name("day")
                        .short("d")
                        .long("day")
                        .value_name("DAY")
                        .help("Download the input file for the day $DAY")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("setup")
                .about("Setup the template file for the day $DAY")
                .arg(Arg::with_name("DAY").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("session", Some(session_args)) => {
            set_session(session_args.value_of("SESSION").unwrap_or(""))
        }
        ("input", Some(input_args)) => match input_args.value_of("day") {
            Some("all") => download_all_inputs(year, src_dir),
            Some(day) => {
                let parsed_day = day.parse::<u32>().expect("Invalid parameter");
                assert!(1 <= parsed_day && parsed_day <= 25, "Invalid parameter");
                get_input(year, day, src_dir);
            }
            None => {
                let today = chrono::offset::Local::now();
                if today.year() == year && today.month() == 12 {
                    get_input(year, &format!("{}", today.day()), src_dir);
                } else {
                    download_all_inputs(year, src_dir);
                };
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
                .open(format!("{}/src/day{}.rs", src_dir, day))
                .expect("Failed to create template file");
            write!(day_file, "{}", TEMPLATE).expect("Failed to write to template file");
        }
        _ => f(matches.value_of("day").or(last_day)),
    }
}

pub fn get_input(year: i32, day: &str, src_dir: &str) -> String {
    if let Ok(input) = std::fs::read_to_string(format!("{}/input/{}/day{}.txt", src_dir, year, day))
    {
        return input;
    }

    let agent = create_agent(get_session());
    download_input(&agent, year, day, src_dir)
}

fn session_file() -> std::path::PathBuf {
    directories::ProjectDirs::from("com.github", "Giuschi", "Aoc-Session")
        .expect("Couldn't find a valid home directory")
        .config_dir()
        .to_path_buf()
}

fn set_session(session: &str) {
    let session_file = session_file();
    if let Some(parent) = session_file.parent() {
        std::fs::create_dir_all(parent).expect("Couldn't create parent directories");
    }
    let mut session_file =
        std::fs::File::create(&session_file).expect("Couldn't create config file");
    write!(session_file, "{}", session).expect("Couldn't write to config file");
}

fn get_session() -> String {
    let session_file = session_file();
    std::fs::read_to_string(&session_file)
        .expect("Couldn't open config file. Did you set it up properly?")
}

fn create_agent(session: String) -> ureq::Agent {
    let mut session_cookie = ureq::Cookie::new("session", session);
    session_cookie.set_domain("adventofcode.com");

    let adventofcode_url =
        url::Url::parse("https://adventofcode.com").expect("adventofcode.com is invalid");

    let mut cookie_store = Default::default();
    // Hack to not depend on cookie_store
    if false {
        ureq::AgentBuilder::new().cookie_store(std::mem::take(&mut cookie_store));
    }
    cookie_store
        .insert_raw(&session_cookie, &adventofcode_url)
        .expect("Failed to set cookie");

    ureq::AgentBuilder::new().cookie_store(cookie_store).build()
}

fn download_input(agent: &ureq::Agent, year: i32, day: &str, src_dir: &str) -> String {
    print!("     - Downloading input for day {:<2}... ", day);

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let body = agent
        .get(&url)
        .call()
        .expect("Input request failed")
        .into_string()
        .expect("Input request failed");

    if body == "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" {
        panic!("Invalid session cookie");
    }

    let destination: std::path::PathBuf =
        format!("{}/input/{}/day{}.txt", src_dir, year, day).into();
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).expect("Couldn't create parent directories");
    }
    let mut file = std::fs::File::create(&destination).expect("Couldn't create input file");
    write!(file, "{}", body).expect("Couldn't write input file");

    println!("Input downloaded");

    body
}

fn download_all_inputs(year: i32, src_dir: &str) {
    let agent = create_agent(get_session());

    let today = chrono::offset::Local::now();
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

        if std::fs::File::open(format!("{}/input/{}/day{}.txt", src_dir, year, day)).is_ok() {
            println!("     - Input already downloaded.");
        } else {
            download_input(&agent, year, &day, src_dir);
        }
    }
}
