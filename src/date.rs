use std::fmt;

struct Date {
	year: u16,
	month: u8,
	day: u8,
}

impl Date {
	pub fn new(strDate: &str) -> Self {
		let parts: Vec<&str> = strDate.split('-').collect();
		return Date {
			year: parts[0].parse().expect("Invalid year"),
			month: parts[1].parse().expect("Invalid month"),
			day: parts[2].parse().expect("Invalid day"),
		};
	}

	pub fn add_days(&self, n: u16) -> Self {
		let new_year: u16;
		let new_month: u8;
		let new_day: u8;
		// TODO: leap-year and all that date arithmetic logic
		return Date {
			year: new_year,
			month: new_month,
			day: new_day,
		};
	}
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day);
    }
}