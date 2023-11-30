/*
Problem 19 - Counting Sundays

You are given the following information, but you may prefer to do some research for yourself.
1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/
fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn main() {
    let mut sundays_on_first = 0;
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut current_day = 1; // January 1, 1901 was a Tuesday, so starting with 1 (Tuesday)

    for year in 1901..=2000 {
        for (month, &days) in days_in_month.iter().enumerate() {
            if month == 1 && is_leap_year(year) {
                current_day = (current_day + 29) % 7; // Leap year
            } else {
                current_day = (current_day + days) % 7; // Regular year
            }

            if current_day == 0 {
                // Checking if the first day of the month is a Sunday (6 represents Sunday)
                sundays_on_first += 1;
            }
        }
    }

    println!("The amount of Sundays that fell on the first of the month during the twentieth century is {}.", sundays_on_first);
}
