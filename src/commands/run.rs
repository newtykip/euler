use owo_colors::OwoColorize;

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
#[path = "../bin/4.rs"]
mod four;
#[path = "../bin/14.rs"]
mod fourteen;
#[path = "../bin/9.rs"]
mod nine;
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
#[path = "../bin/10.rs"]
mod ten;
#[path = "../bin/13.rs"]
mod thirteen;
#[path = "../bin/3.rs"]
mod three;
#[path = "../bin/12.rs"]
mod twelve;
#[path = "../bin/2.rs"]
mod two;
// #[path = "../bin/19.rs"]
// mod nineteen;
// #[path = "../bin/20.rs"]
// mod twenty;
// #[path = "../bin/21.rs"]
// mod twenty_one;
#[path = "../bin/22.rs"]
mod twenty_two;
// #[path = "../bin/23.rs"]
// mod twenty_three;
// #[path = "../bin/24.rs"]
// mod twenty_four;
// #[path = "../bin/25.rs"]
// mod twenty_five;
// #[path = "../bin/26.rs"]
// mod twenty_six;
// #[path = "../bin/27.rs"]
// mod twenty_seven;
// #[path = "../bin/28.rs"]
// mod twenty_eight;
// #[path = "../bin/29.rs"]
// mod twenty_nine;
// #[path = "../bin/30.rs"]
// mod thirty;
// #[path = "../bin/31.rs"]
// mod thirty_one;
// #[path = "../bin/32.rs"]
// mod thirty_two;
// #[path = "../bin/33.rs"]
// mod thirty_three;
// #[path = "../bin/34.rs"]
// mod thirty_four;
// #[path = "../bin/35.rs"]
// mod thirty_five;
// #[path = "../bin/36.rs"]
// mod thirty_six;
// #[path = "../bin/37.rs"]
// mod thirty_seven;
// #[path = "../bin/38.rs"]
// mod thirty_eight;
// #[path = "../bin/39.rs"]
// mod thirty_nine;
// #[path = "../bin/40.rs"]
// mod forty;
// #[path = "../bin/41.rs"]
// mod forty_one;
#[path = "../bin/42.rs"]
mod forty_two;
// #[path = "../bin/43.rs"]
// mod forty_three;
// #[path = "../bin/44.rs"]
// mod forty_four;
// #[path = "../bin/45.rs"]
// mod forty_five;
// #[path = "../bin/46.rs"]
// mod forty_six;
// #[path = "../bin/47.rs"]
// mod forty_seven;
// #[path = "../bin/48.rs"]
// mod forty_eight;
// #[path = "../bin/49.rs"]
// mod forty_nine;
// #[path = "../bin/50.rs"]
// mod fifty;
// #[path = "../bin/51.rs"]
// mod fifty_one;
// #[path = "../bin/52.rs"]
// mod fifty_two;
// #[path = "../bin/53.rs"]
// mod fifty_three;
// #[path = "../bin/54.rs"]
// mod fifty_four;
// #[path = "../bin/55.rs"]
// mod fifty_five;
// #[path = "../bin/56.rs"]
// mod fifty_six;
// #[path = "../bin/57.rs"]
// mod fifty_seven;
// #[path = "../bin/58.rs"]
// mod fifty_eight;
// #[path = "../bin/59.rs"]
// mod fifty_nine;
// #[path = "../bin/60.rs"]
// mod sixty;
// #[path = "../bin/61.rs"]
// mod sixty_one;
// #[path = "../bin/62.rs"]
// mod sixty_two;
// #[path = "../bin/63.rs"]
// mod sixty_three;
// #[path = "../bin/64.rs"]
// mod sixty_four;
// #[path = "../bin/65.rs"]
// mod sixty_five;
// #[path = "../bin/66.rs"]
// mod sixty_six;
#[path = "../bin/67.rs"]
mod sixty_seven;
// #[path = "../bin/68.rs"]
// mod sixty_eight;
// #[path = "../bin/69.rs"]
// mod sixty_nine;
// #[path = "../bin/70.rs"]
// mod seventy;
// #[path = "../bin/71.rs"]
// mod seventy_one;
// #[path = "../bin/72.rs"]
// mod seventy_two;
// #[path = "../bin/73.rs"]
// mod seventy_three;
// #[path = "../bin/74.rs"]
// mod seventy_four;
// #[path = "../bin/75.rs"]
// mod seventy_five;
// #[path = "../bin/76.rs"]
// mod seventy_six;
// #[path = "../bin/77.rs"]
// mod seventy_seven;
// #[path = "../bin/78.rs"]
// mod seventy_eight;
// #[path = "../bin/79.rs"]
// mod seventy_nine;
// #[path = "../bin/80.rs"]
// mod eighty;
// #[path = "../bin/81.rs"]
// mod eighty_one;
// #[path = "../bin/82.rs"]
// mod eighty_two;
// #[path = "../bin/83.rs"]
// mod eighty_three;
// #[path = "../bin/84.rs"]
// mod eighty_four;
// #[path = "../bin/85.rs"]
// mod eighty_five;
// #[path = "../bin/86.rs"]
// mod eighty_six;
// #[path = "../bin/87.rs"]
// mod eighty_seven;
// #[path = "../bin/88.rs"]
// mod eighty_eight;
// #[path = "../bin/89.rs"]
// mod eighty_nine;
// #[path = "../bin/90.rs"]
// mod ninety;
// #[path = "../bin/91.rs"]
// mod ninety_one;
// #[path = "../bin/92.rs"]
// mod ninety_two;
// #[path = "../bin/93.rs"]
// mod ninety_three;
// #[path = "../bin/94.rs"]
// mod ninety_four;
// #[path = "../bin/95.rs"]
// mod ninety_five;
// #[path = "../bin/96.rs"]
// mod ninety_six;
// #[path = "../bin/97.rs"]
// mod ninety_seven;
// #[path = "../bin/98.rs"]
// mod ninety_eight;
// #[path = "../bin/99.rs"]
// mod ninety_nine;
// #[path = "../bin/100.rs"]
// mod one_hundred;

#[tokio::main]
pub async fn execute(
    problem: Option<u8>,
    benchmark: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let code_path = euler::code_path();
    let problem_number =
        euler::problem_number(problem, &code_path, "Which problem would you like to run?");

    let start = std::time::Instant::now();
    let mut exists = true;

    match problem_number {
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
        // 19 => nineteen::main(),
        // 20 => twenty::main(),
        // 21 => twenty_one::main(),
        22 => twenty_two::main(),
        // 23 => twenty_three::main(),
        // 24 => twenty_four::main(),
        // 25 => twenty_five::main(),
        // 26 => twenty_six::main(),
        // 27 => twenty_seven::main(),
        // 28 => twenty_eight::main(),
        // 29 => twenty_nine::main(),
        // 30 => thirty::main(),
        // 31 => thirty_one::main(),
        // 32 => thirty_two::main(),
        // 33 => thirty_three::main(),
        // 34 => thirty_four::main(),
        // 35 => thirty_five::main(),
        // 36 => thirty_six::main(),
        // 37 => thirty_seven::main(),
        // 38 => thirty_eight::main(),
        // 39 => thirty_nine::main(),
        // 40 => forty::main(),
        // 41 => forty_one::main(),
        // 42 => forty_two::main(),
        // 43 => forty_three::main(),
        // 44 => forty_four::main(),
        // 45 => forty_five::main(),
        // 46 => forty_six::main(),
        // 47 => forty_seven::main(),
        // 48 => forty_eight::main(),
        // 49 => forty_nine::main(),
        // 50 => fifty::main(),
        // 51 => fifty_one::main(),
        // 52 => fifty_two::main(),
        // 53 => fifty_three::main(),
        // 54 => fifty_four::main(),
        // 55 => fifty_five::main(),
        // 56 => fifty_six::main(),
        // 57 => fifty_seven::main(),
        // 58 => fifty_eight::main(),
        // 59 => fifty_nine::main(),
        // 60 => sixty::main(),
        // 61 => sixty_one::main(),
        // 62 => sixty_two::main(),
        // 63 => sixty_three::main(),
        // 64 => sixty_four::main(),
        // 65 => sixty_five::main(),
        // 66 => sixty_six::main(),
        67 => sixty_seven::main(),
        // 68 => sixty_eight::main(),
        // 69 => sixty_nine::main(),
        // 70 => seventy::main(),
        // 71 => seventy_one::main(),
        // 72 => seventy_two::main(),
        // 73 => seventy_three::main(),
        // 74 => seventy_four::main(),
        // 75 => seventy_five::main(),
        // 76 => seventy_six::main(),
        // 77 => seventy_seven::main(),
        // 78 => seventy_eight::main(),
        // 79 => seventy_nine::main(),
        // 80 => eighty::main(),
        // 81 => eighty_one::main(),
        // 82 => eighty_two::main(),
        // 83 => eighty_three::main(),
        // 84 => eighty_four::main(),
        // 85 => eighty_five::main(),
        // 86 => eighty_six::main(),
        // 87 => eighty_seven::main(),
        // 88 => eighty_eight::main(),
        // 89 => eighty_nine::main(),
        // 90 => ninety::main(),
        // 91 => ninety_one::main(),
        // 92 => ninety_two::main(),
        // 93 => ninety_three::main(),
        // 94 => ninety_four::main(),
        // 95 => ninety_five::main(),
        // 96 => ninety_six::main(),
        // 97 => ninety_seven::main(),
        // 98 => ninety_eight::main(),
        // 99 => ninety_nine::main(),
        // 100 => one_hundred::main(),
        _ => {
            exists = false;
            println!(
                "{} {} {}",
                "Problem".red().bold(),
                problem_number.red().bold(),
                "is not in this repository!".red().bold()
            )
        }
    }

    let duration = start.elapsed();

    if benchmark && exists {
        println!(
            "Time elapsed when executing problem {} is: {:?}",
            problem_number, duration
        );
    }

    Ok(())
}
