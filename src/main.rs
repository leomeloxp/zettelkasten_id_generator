use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use clap::Parser;
use radix_fmt::radix;

#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None)]
struct Cli {
    /// A value to be used when generating or parsing a zID
    input: Option<String>,
    /// The radix base used for conversion between 10 and 36
    #[clap(default_value = "36")]
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(10..37))]
    radix: u8,

    /// Generates a zID from a supplied date
    #[arg(long, requires = "input")]
    from_date: bool,

    /// Generates a zID from a supplied date in ISO format
    /// eg, 2001-01-01T07:35:42
    #[arg(long, requires = "input")]
    from_iso_date: bool,

    /// Converts a zID back to date in the format "YYYY-MM-DD"
    #[arg(long, requires = "input")]
    to_date: bool,

    /// Converts a zID back to date in the format "YYYY-MM-DDTHH:MM:SS"
    #[arg(long, requires = "input")]
    to_iso_date: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let base = get_base_radix(args.radix)?;

    let input = if args.from_date || args.from_iso_date || args.to_date {
        args.input.expect("No input provided").trim().to_owned()
    } else {
        "".to_owned()
    };

    let zid = if args.from_date {
        zid_from_date(base, input)
    } else if args.from_iso_date {
        zid_from_iso_date(base, input)
    } else if args.to_date {
        zid_to_date(base, input)
    } else if args.to_iso_date {
        zid_to_iso_date(base, input)
    } else {
        now_to_zid(base)
    }
    .expect("Failed to generate output");

    print!("{}", zid);

    Ok(())
}

fn get_base_radix(input: u8) -> Result<u8> {
    let max_base = 36;
    let min_base = 10;

    Ok(min_base.max(max_base.min(input)))
}

/// Generates a Zettelkasten ID using the current date and time (to the second).
fn now_to_zid(base: u8) -> Result<String> {
    let unix_time = Utc::now().timestamp();
    let zid = radix(unix_time, base);
    Ok(zid.to_string())
}

/// Generates a Zettelkasten ID from a Date.

fn zid_from_date(base: u8, input: String) -> Result<String> {
    let date = NaiveDate::parse_from_str(&input, "%Y-%m-%d").expect("Failed to parse date");
    let datetime = date.and_hms_opt(0, 0, 0).expect("Failed to parse date");
    let unix_time = Utc.from_local_datetime(&datetime).unwrap().timestamp();

    let zid = radix(unix_time as u64, base);

    Ok(zid.to_string())
}

/// Generates a Zettelkasten ID from an ISO8601 formatted Date.
fn zid_from_iso_date(base: u8, input: String) -> Result<String> {
    let datetime =
        NaiveDateTime::parse_from_str(&input, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date");

    let unix_time = Utc.from_local_datetime(&datetime).unwrap().timestamp();

    let zid = radix(unix_time as u64, base);

    Ok(zid.to_string())
}

/// Returns the YYYY-MM-DD date string from a supplied zID.
fn zid_to_date(base: u8, input: String) -> Result<String> {
    let timestamp = u64::from_str_radix(&input, base as u32).expect("Failed to parse input");
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).expect("Failed to parse zid");
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    Ok(datetime.format("%Y-%m-%d").to_string())
}

/// Returns the YYYY-MM-DDTHH:MM:SS date string from a supplied zID.
fn zid_to_iso_date(base: u8, input: String) -> Result<String> {
    let timestamp = u64::from_str_radix(&input, base as u32).expect("Failed to parse input");
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).expect("Failed to parse zid");
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    Ok(datetime.format("%Y-%m-%dT%H:%M:%S").to_string())
}
