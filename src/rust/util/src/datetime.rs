// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use time::{OffsetDateTime, UtcOffset, Weekday};

pub struct Tm {
    datetime: OffsetDateTime,
}

impl Tm {
    /// Returns the weekday, one of "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
    pub fn weekday(&self) -> String {
        match self.datetime.weekday() {
            Weekday::Sunday => "Sun",
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
        }
        .into()
    }

    /// Returns 'year-month-day', e.g. 2024-07-16, equivalent of '%Y-%m-%d' in strftime.
    pub fn date(&self) -> String {
        let date = self.datetime.date();
        format!(
            "{}-{:02}-{:02}",
            date.year(),
            date.month() as u8,
            date.day(),
        )
    }

    /// Returns the zero-padded hour from 00-23, e.g. "07".
    pub fn hour(&self) -> String {
        format!("{:02}", self.datetime.hour())
    }

    /// Returns the zero-padded minute from 00-59, e.g. "07".
    pub fn minute(&self) -> String {
        format!("{:02}", self.datetime.minute())
    }

    /// Returns the zero-padded second from 00-60, e.g. "07".
    pub fn second(&self) -> String {
        format!("{:02}", self.datetime.second())
    }
}

pub fn get_datetime(timestamp: u32) -> Result<Tm, ()> {
    let datetime = OffsetDateTime::from_unix_timestamp(timestamp as i64).map_err(|_| ())?;
    Ok(Tm { datetime })
}

/// Formats the timestamp in the local timezone.
/// timestamp is the unix timestamp in seconds.
/// timezone_offset is added to the timestamp, timezone part.
/// date_only: if true, only the date is formatted. If false, both date and time are.
pub fn format_datetime(
    timestamp: u32,
    timezone_offset: i32,
    date_only: bool,
) -> Result<String, ()> {
    const MAX_EAST_UTC_OFFSET: i32 = 50400; // 14 hours in seconds
    const MAX_WEST_UTC_OFFSET: i32 = -43200; // 12 hours in seconds

    if !(MAX_WEST_UTC_OFFSET..=MAX_EAST_UTC_OFFSET).contains(&timezone_offset) {
        return Err(());
    }

    let offset = UtcOffset::from_whole_seconds(timezone_offset).map_err(|_| ())?;
    let datetime = OffsetDateTime::from_unix_timestamp(timestamp as i64)
        .map_err(|_| ())?
        .checked_to_offset(offset)
        .ok_or(())?;
    let tm = Tm { datetime };
    Ok(if date_only {
        format!("{} {}", tm.weekday(), tm.date())
    } else {
        format!(
            "{} {}\n{}:{}",
            tm.weekday(),
            tm.date(),
            tm.hour(),
            tm.minute()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime() {
        assert_eq!(
            format_datetime(1601281809, 0, true),
            Ok("Mon 2020-09-28".into())
        );
        assert_eq!(
            format_datetime(1601281809, 0, false),
            Ok("Mon 2020-09-28\n08:30".into()),
        );
        assert_eq!(
            format_datetime(1601281809, 18000, false),
            Ok("Mon 2020-09-28\n13:30".into()),
        );
        assert_eq!(
            format_datetime(1601281809, -32400, false),
            Ok("Sun 2020-09-27\n23:30".into()),
        );

        assert!(format_datetime(1601281809, 50401, false).is_err());
        assert!(format_datetime(1601281809, -43201, false).is_err());
    }
}
