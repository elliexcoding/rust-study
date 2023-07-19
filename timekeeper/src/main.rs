#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

use chrono::DateTime;
use chrono::Local;
use clap::{App, Arg};
use std::mem::zeroed;

struct Clock;


const NTP_TO_UNIX_SECONDS: u64 = 2_208_988_800;
#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
  seconds: u32,
  fraction: u32,
}

impl From<NTPTimestamp> for DateTime<Utc> {
  fn from(ntp: NTPTimestamp) -> Self {
    let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS as i64;
    let mut nanos = ntp.fraction as f64;
    nanos *= 1e9;
    nanos /= 2_f64.powi(32);

    Utc.timestamp(secs, nanos as u32)
  }
}



impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};
        let t = t.with_timezone(&Local);
        let mut systime: SYSTEMTIME = unsafe { zeroed() };
        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns == 1_000_000_000;
        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }


    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
    use libc::{settimeofday, timezone};
  
      let t = t.with_timezone(&Local);
      let mut u: timeval = unsafe { zeroed() };
  
      u.tv_sec = t.timestamp() as time_t;
      u.tv_usec =
        t.timestamp_subsec_micros() as suseconds_t;
  
      unsafe {
        let mock_tz: *const timezone = std::ptr::null();
        settimeofday(&u as *const timeval, mock_tz);
      }
    }
}

fn main() {
    let app = App::new("clock")
      .version("0.1.2")
      .about("Gets and (aspirationally) sets the time.")
      .after_help(
        "Note: UNIX timestamps are parsed as whole \
         seconds since 1st January 1970 0:00:00 UTC. \
         For more accuracy, use another format.",
      )
      .arg(
        Arg::with_name("action")
          .takes_value(true)
          .possible_values(&["get", "set"])
          .default_value("get"),
      )
      .arg(
        Arg::with_name("std")
          .short("s")
          .long("use-standard")
          .takes_value(true)
          .possible_values(&[
            "rfc2822",
            "rfc3339",
            "timestamp",
          ])
          .default_value("rfc3339"),
      )
      .arg(Arg::with_name("datetime").help(
        "When <action> is 'set', apply <datetime>. \
         Otherwise, ignore.",
      ));
  
    let args = app.get_matches();
  
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();
  
    if action == "set" {
      let t_ = args.value_of("datetime").unwrap();
  
      let parser = match std {
        "rfc2822" => DateTime::parse_from_rfc2822,
        "rfc3339" => DateTime::parse_from_rfc3339,
        _ => unimplemented!(),
      };
  
      let err_msg = format!(
        "Unable to parse {} according to {}",
        t_, std
      );
      let t = parser(t_).expect(&err_msg);
  
      Clock::set(t);

      let maybe_error = 
      std::io::Error::last_os_error();
      let os_error_code = 
      &maybe_error.raw_os_error();

      match os_error_code {
        Some(0) => (),
        Some(_) => {
          eprintln!("Warning: {}", maybe_error);
        },
        None => (),
      }
    }
  
    let now = Clock::get();
  
    match std {
      "timestamp" => println!("{}", now.timestamp()),
      "rfc2822" => println!("{}", now.to_rfc2822()),
      "rfc3339" => println!("{}", now.to_rfc3339()),
      _ => unreachable!(),
    }


    fn check_time() -> Result<f64, std::io::Error> {
      const NTP_PORT: u16 = 123;

      let servers = [
        "time.nist.gov",
        "time.apple.com",
        "time.euro.apple.com",
        "time.google.com",
        "time2.google.com",
      ];

      let mut times = Vec::with_capacity(servers.len);

      for &server in servers.iter() {
        print!("{}: ", server);
      }

      let calc = ntp_roundtirp(&server, NTP_PORT);

      match calc {
        Ok(time) => {
          println!(" {} ms away from local system time", time.offset());
          time.push(time);
        }
        Err(_) => {
          println!(" ? [response took too long]")
        }
      };

      let mut offsets = Vec::with_capacity(servers.len());
      let mut offset_weights = Vec::with_capacity(servers.len());

      for time in &times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;
        
        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
          offsets.push(offset);
          offset_weights.push(weight);
        }
      }

      let avg_offset = weighted_mean(&offsets, &offset_weights);

      Ok(avg_offset)
    }
}
