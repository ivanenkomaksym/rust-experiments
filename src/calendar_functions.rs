const DAYTAB: [[u32; 13]; 2] = [
    [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

pub fn day_of_year(year: u32, month: u32, day: u32) -> u32 {
    let mut day_of_year = day;
    let leap = year % 4 == 0 && year % 100 != 0 || year % 400 == 0;
    for i in 1..month {
        day_of_year += DAYTAB[leap as usize][i as usize];
    }
    day_of_year
}

pub fn month_day(year: u32, yearday: u32) -> (u32, u32) {
    let leap = year % 4 == 0 && year % 100 != 0 || year % 400 == 0;
    let mut month = 1;
    let mut day = yearday;

    while day > DAYTAB[leap as usize][month as usize] {
        day -= DAYTAB[leap as usize][month as usize];
        month += 1;
    }

    (month, day)
}