// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use time::{OffsetDateTime, UtcOffset, Weekday};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DateLocale {
    English,
    German,
}

pub struct Tm {
    datetime: OffsetDateTime,
}

impl Tm {
    /// Returns the weekday, one of "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
    pub fn weekday(&self) -> String {
        self.weekday_locale(DateLocale::English)
    }

    fn weekday_locale(&self, locale: DateLocale) -> String {
        match self.datetime.weekday() {
            Weekday::Sunday => match locale {
                DateLocale::English => "Sun",
                DateLocale::German => "So",
            },
            Weekday::Monday => match locale {
                DateLocale::English => "Mon",
                DateLocale::German => "Mo",
            },
            Weekday::Tuesday => match locale {
                DateLocale::English => "Tue",
                DateLocale::German => "Di",
            },
            Weekday::Wednesday => match locale {
                DateLocale::English => "Wed",
                DateLocale::German => "Mi",
            },
            Weekday::Thursday => match locale {
                DateLocale::English => "Thu",
                DateLocale::German => "Do",
            },
            Weekday::Friday => match locale {
                DateLocale::English => "Fri",
                DateLocale::German => "Fr",
            },
            Weekday::Saturday => match locale {
                DateLocale::English => "Sat",
                DateLocale::German => "Sa",
            },
        }
        .into()
    }

    /// Returns 'year-month-day', e.g. 2024-07-16, equivalent of '%Y-%m-%d' in strftime.
    pub fn date(&self) -> String {
        self.date_locale(DateLocale::English)
    }

    fn date_locale(&self, locale: DateLocale) -> String {
        let date = self.datetime.date();
        match locale {
            DateLocale::English => format!(
                "{}-{:02}-{:02}",
                date.year(),
                date.month() as u8,
                date.day(),
            ),
            DateLocale::German => format!(
                "{:02}.{:02}.{}",
                date.day(),
                date.month() as u8,
                date.year(),
            ),
        }
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
    format_datetime_locale(timestamp, timezone_offset, date_only, DateLocale::English)
}

pub fn format_datetime_locale(
    timestamp: u32,
    timezone_offset: i32,
    date_only: bool,
    locale: DateLocale,
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
        format!("{} {}", tm.weekday_locale(locale), tm.date_locale(locale))
    } else {
        format!(
            "{} {}\n{}:{}",
            tm.weekday_locale(locale),
            tm.date_locale(locale),
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

    #[test]
    fn test_format_datetime_locale() {
        assert_eq!(
            format_datetime_locale(1779964200, 0, true, DateLocale::German),
            Ok("Do 28.05.2026".into())
        );
        assert_eq!(
            format_datetime_locale(1779964200, 0, false, DateLocale::German),
            Ok("Do 28.05.2026\n10:30".into())
        );
        assert_eq!(
            format_datetime_locale(1601281809, -32400, false, DateLocale::German),
            Ok("So 27.09.2020\n23:30".into())
        );
    }
}
