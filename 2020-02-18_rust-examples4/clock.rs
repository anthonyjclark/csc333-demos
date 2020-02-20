use std::fmt;
use std::time::SystemTime;

#[derive(Debug)]
enum Period {
    AM,
    PM,
}

#[derive(Debug)]
struct Clock {
    hour: u8,
    minute: u8,
    period: Period,
}

impl Clock {
    fn constructor(h: u8, m: u8, p: Period) -> Clock {
        Clock {
            hour: h,
            minute: m,
            period: p,
        }
    }

    // fn new((h, m, p): (u8, u8, Period)) -> Clock {
    //     Clock {
    //         hour: h,
    //         minute: m,
    //         period: p,
    //     }
    // }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let period_string = match &self.period {
            Period::AM => "in the morning",
            Period::PM => "in the other parts of the day"
        };
        write!(f, "{}:{:02} {}", self.hour, self.minute, period_string)
    }
}

impl fmt::Display for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "blank")
    }
}

fn main() {
    let current_clock = Clock::constructor(10, 5, Period::AM);
    println!("{:02}", current_clock);

    let time = SystemTime::now();
    println!("{}", time);
}
