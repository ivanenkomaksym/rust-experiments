
#[cfg(test)]
mod tests {
    use rust_experiments::calendar_functions::day_of_year;
    use rust_experiments::calendar_functions::month_day;

    #[test]
    fn test_day_of_year() {
        assert_eq!(day_of_year(2021, 1, 1), 1);
        assert_eq!(day_of_year(2021, 12, 31), 365);
        assert_eq!(day_of_year(2020, 1, 1), 1);
        assert_eq!(day_of_year(2020, 12, 31), 366);
        assert_eq!(day_of_year(2020, 2, 29), 60);
        assert_eq!(day_of_year(2019, 3, 1), 60);
        assert_eq!(day_of_year(2019, 12, 31), 365);
    }

    #[test]
    fn test_leap_years() {
        assert_eq!(day_of_year(2000, 2, 29), 60); // Leap year divisible by 400
        assert_eq!(day_of_year(1900, 3, 1), 60); // Not a leap year, divisible by 100 but not 400
        assert_eq!(day_of_year(2004, 3, 1), 61); // Leap year divisible by 4
        assert_eq!(day_of_year(2001, 3, 1), 60); // Not a leap year
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(day_of_year(2021, 1, 31), 31);
        assert_eq!(day_of_year(2021, 2, 28), 59);
        assert_eq!(day_of_year(2020, 2, 28), 59);
        assert_eq!(day_of_year(2020, 2, 29), 60);
        assert_eq!(day_of_year(2020, 3, 1), 61);
    }

    #[test]
    fn test_month_day() {
        assert_eq!(month_day(2021, 1), (1, 1));
        assert_eq!(month_day(2021, 31), (1, 31));
        assert_eq!(month_day(2021, 59), (2, 28));
        assert_eq!(month_day(2020, 60), (2, 29));
        assert_eq!(month_day(2020, 61), (3, 1));
    }
}