use euler::Problem;
use owo_colors::OwoColorize;
use std::time::Instant;

#[path = "../bin/8.rs"]
mod eight;
#[path = "../bin/18.rs"]
mod eighteen;
#[path = "../bin/11.rs"]
mod eleven;
#[path = "../bin/15.rs"]
mod fifteen;
#[path = "../bin/5.rs"]
mod five;
#[path = "../bin/42.rs"]
mod forty_two;
#[path = "../bin/4.rs"]
mod four;
#[path = "../bin/14.rs"]
mod fourteen;
#[path = "../bin/9.rs"]
mod nine;
#[path = "../bin/19.rs"]
mod nineteen;
#[path = "../bin/1.rs"]
mod one;
#[path = "../bin/7.rs"]
mod seven;
#[path = "../bin/17.rs"]
mod seventeen;
#[path = "../bin/6.rs"]
mod six;
#[path = "../bin/16.rs"]
mod sixteen;
#[path = "../bin/67.rs"]
mod sixty_seven;
#[path = "../bin/10.rs"]
mod ten;
#[path = "../bin/13.rs"]
mod thirteen;
#[path = "../bin/3.rs"]
mod three;
#[path = "../bin/12.rs"]
mod twelve;
#[path = "../bin/20.rs"]
mod twenty;
#[path = "../bin/27.rs"]
mod twenty_seven;
#[path = "../bin/22.rs"]
mod twenty_two;
#[path = "../bin/2.rs"]
mod two;

pub async fn execute(
    problem: Option<u8>,
    benchmark: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let problem = problem
        .unwrap_or_else(|| Problem::prompt_number("Please select a problem:", true).unwrap());
    let mut exists = true; // assume it exists

    let start = if benchmark {
        Some(Instant::now())
    } else {
        None
    };

    // execute the solution
    match problem {
        1 => one::main(),
        2 => two::main(),
        3 => three::main(),
        4 => four::main(),
        5 => five::main(),
        6 => six::main(),
        7 => seven::main(),
        8 => eight::main(),
        9 => nine::main(),
        10 => ten::main(),
        11 => eleven::main(),
        12 => twelve::main(),
        13 => thirteen::main(),
        14 => fourteen::main(),
        15 => fifteen::main(),
        16 => sixteen::main(),
        17 => seventeen::main(),
        18 => eighteen::main(),
        22 => twenty_two::main(),
        27 => twenty_seven::main(),
        67 => sixty_seven::main(),
        19 => nineteen::main(),
        20 => twenty::main(),
        _ => {
            exists = false;
            println!(
                "{}",
                format!("Problem #{problem} is yet to be solved!")
                    .red()
                    .bold()
            )
        }
    }

    // if the script exists, print the time elapsed
    if benchmark && exists {
        let duration = start.unwrap().elapsed();

        println!(
            "
Time elapsed when executing problem {} is: {:?}",
            problem, duration
        );
    }

    Ok(())
}
