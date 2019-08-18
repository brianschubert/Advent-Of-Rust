//! Analysis of guard napping behavior during late-night shifts.

use chrono::{NaiveDateTime, Timelike};
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

/// The integer type used to represent a Guard's unique ID.
pub type GuardId = u16;

/// The integer type used to occurrences of an event.
type Counter = u16;

/// The integer type used to represent a minute..
type Minute = usize;

/// Type used to count occurrences of events associated with different guards.
type GuardCounter = HashMap<GuardId, Counter>;

#[derive(Debug, Eq, PartialEq)]
/// The action associated with an entry in a guard log.
enum GuardAction {
    Begin(GuardId),
    Sleep,
    Wake,
}

#[derive(Debug, PartialEq, Eq)]
/// A one-line entry in the guard log.
///
/// This structure is used as an intermediate representation of the puzzle
/// input for sorting the log entries by timestamp and resolving which
/// log entries apply to which guard. After this initial analysis, all
/// `GuardLogEntry`s are condensed  into `Shift` instances.
struct GuardLogEntry {
    timestamp: NaiveDateTime,
    action: GuardAction,
}

impl GuardLogEntry {
    const DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M";

    /// Returns the ID of the guard associated with this entry if this entry
    /// denotes the beginning of a new shift. Otherwise, returns `None`.
    pub fn guard_id(&self) -> Option<GuardId> {
        match self.action {
            GuardAction::Begin(id) => Some(id),
            _ => None,
        }
    }

    /// Returns `true` if this entry denotes the beginning of a new shift.
    pub fn is_shift_beginning(&self) -> bool {
        self.guard_id().is_some()
    }
}

impl FromStr for GuardLogEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const BEGIN_TIMESTAMP: usize = 1;
        const END_TIMESTAMP: usize = 17;
        let timestamp = NaiveDateTime::parse_from_str(
            &s[BEGIN_TIMESTAMP..END_TIMESTAMP],
            Self::DATE_TIME_FORMAT,
        )
        .map_err(|e| {
            format!(
                "failed to parse guard log entry: bad timestamp - {}",
                e.description()
            )
        })?;

        const ACTION_BYTE: usize = 19;
        const BEGIN_ID: usize = 26;
        let action = match s.as_bytes()[ACTION_BYTE] {
            b'G' => {
                let end = s[BEGIN_ID..]
                    .find(' ')
                    .ok_or("failed to parse guard log entry: missing guard number")?;
                GuardAction::Begin(
                    s[BEGIN_ID..BEGIN_ID + end]
                        .parse()
                        .map_err(|_| "failed to parse guard log entry: malformed guard number")?,
                )
            }
            b'f' => GuardAction::Sleep,
            b'w' => GuardAction::Wake,
            c => {
                return Err(format!(
                    "failed to parse guard log entry: unknown guard action beginning with '{}'",
                    c as char,
                ));
            }
        };
        Ok(GuardLogEntry { timestamp, action })
    }
}

impl PartialOrd for GuardLogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for GuardLogEntry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// A log of guard activity.
pub struct GuardLog {
    /// Log of the distinct guard shifts that occur.
    shift_log: Vec<Shift>,
    /// Log of how many times each particular guard is asleep during a
    /// given minute of the midnight hour.
    ///
    /// Guaranteed to have length 60.
    minute_log: Box<[GuardCounter]>,
}

impl GuardLog {
    const MINUTES_PER_HOUR: usize = 60;

    /// Attempts to parse a sequence of log lines into a `GuardLog`.
    pub fn parse_lines<S: AsRef<str>>(lines: &[S]) -> Result<Self, String> {
        let mut entries: Vec<GuardLogEntry> = lines
            .iter()
            .map(|l| l.as_ref().parse())
            .collect::<Result<_, _>>()?;
        entries.sort_unstable();

        let shift_log: Vec<Shift> = entries
            .split(|entry| entry.is_shift_beginning())
            .skip(1) // Skip empty initial slice
            .zip(entries.iter().filter(|entry| entry.is_shift_beginning()))
            .map(|(nap_entries, shift_begin)| {
                let guard_id = shift_begin.guard_id().unwrap(); // Guaranteed to be Some(id)
                Shift::from_guard_and_entries(guard_id, nap_entries).map_err(str::to_owned)
            })
            .collect::<Result<_, _>>()?;

        let mut minute_log = vec![GuardCounter::new(); Self::MINUTES_PER_HOUR].into_boxed_slice();
        for shift in shift_log.iter() {
            for nap in shift.naps.iter() {
                for minute in nap.start.minute()..nap.end.minute() {
                    let entry = minute_log[minute as usize].entry(shift.guard).or_default();
                    *entry += 1;
                }
            }
        }

        Ok(Self {
            shift_log,
            minute_log,
        })
    }

    /// Returns a tuple containing 1) the ID of the guard who slept for the most
    /// minutes and 2) the minute during which that guard slept the most.
    pub fn compute_most_sleepy_guard(&self) -> (GuardId, Minute) {
        let mut guard_nap_time: HashMap<GuardId, Duration> = HashMap::new();

        for shift in self.shift_log.iter() {
            // Compute the duration of time that each guard was asleep
            let nap_total = shift.naps.iter().map(Nap::duration).sum();
            *guard_nap_time.entry(shift.guard).or_default() += nap_total;
        }

        let (most_sleepy_guard, _) = guard_nap_time
            .into_iter()
            // Find the guard who was asleep for the greatest duration of time.
            .max_by_key(|pair| pair.1)
            .expect("guard log MUST contain at least one shift");

        let minute_most_slept = self
            .guard_minute_most_asleep(most_sleepy_guard)
            .unwrap() // most sleep guard is guaranteed to have slept during at least one minute
            .0; // Get the index of the most-slept-in minute

        (most_sleepy_guard, minute_most_slept)
    }

    /// Returns a tuple containing 1) the minute that the given Guard is most
    /// often asleep during and 2) the number of times that Guard has been
    /// asleep during that minute.
    ///
    /// If there is no record of the given Guard sleeping in this log,
    /// `None` is returned.
    fn guard_minute_most_asleep(&self, guard: GuardId) -> Option<(Minute, Counter)> {
        self.minute_log
            .iter()
            .enumerate()
            // Map each minute's Guard-to-times-asleep mapping into 1) the
            // zero-based index of the minute and 2) the number of times
            // that the specified guard has slept during that minute.
            //
            // Filter out minutes that the guard did not sleep during.
            .filter_map(|(minute_index, guard_map)| {
                guard_map
                    .get(&guard)
                    .map(|&times_slept| (minute_index, times_slept))
            })
            .max_by_key(|&pair| pair.1)
    }

    /// Returns a tuple containing 1) the guard that is most often asleep
    /// during the same minute and 2) the minute during which that guard
    /// is most often asleep.
    pub fn compute_guard_most_frequently_asleep_same_minute(&self) -> (GuardId, Minute) {
        self.minute_log
            .iter()
            .enumerate()
            // Map each Guard-to-times-asleep mapping into a tuple of 1) the guard
            // who slept the most during the minute and 2) the number of times
            // they slept during that minute.
            //
            // Filter out minutes during which no guard slept.
            .filter_map(|(minute_index, minute_map)| {
                minute_map
                    .iter()
                    .max_by_key(|&(_guard, &times_asleep)| times_asleep)
                    .map(|pair| (minute_index, pair))
            })
            .max_by_key(|&(_index, (_guard, &times_asleep))| times_asleep)
            .map(|(minute_index, (&guard_id, _times_asleep))| (guard_id, minute_index))
            .expect("minute log MUST NOT be empty")
    }
}

#[derive(Debug, PartialEq, Eq)]
/// A span of time during which a guard is asleep.
struct Nap {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Nap {
    /// Returns the duration between this `Nap`'s start and end times.
    fn duration(&self) -> Duration {
        self.end
            .signed_duration_since(self.start)
            .to_std()
            .expect("nap end time occurred before nap start")
    }
}

#[derive(Debug)]
/// A single shift associating a guard with the naps they take.
struct Shift {
    guard: GuardId,
    naps: Vec<Nap>,
}

impl Shift {
    /// Attempts to parse a sequence of `GuardLogEntry`s into a shift
    /// associated with the specified guard.
    fn from_guard_and_entries(
        guard: GuardId,
        log_entries: &[GuardLogEntry],
    ) -> Result<Self, &'static str> {
        use GuardAction::*;
        let naps = log_entries
            .chunks(2)
            // Match 2-element slice with sleep-wake entry pair
            .map(|naps_parts| match *naps_parts {
                [GuardLogEntry {
                    timestamp: start,
                    action: Sleep,
                }, GuardLogEntry {
                    timestamp: end,
                    action: Wake,
                }] => Ok(Nap { start, end }),
                _ => Err("guard log entries MUST contain alternate sleep-wake entry pairs"),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { guard, naps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use super::super::EXAMPLE_INPUT;

    #[test]
    fn parse_guard_log_entry() {
        assert_eq!(
            "[1518-11-01 00:00] Guard #10 begins shift"
                .parse::<GuardLogEntry>()
                .unwrap(),
            GuardLogEntry {
                action: GuardAction::Begin(10),
                timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
            }
        )
    }

    #[test]
    fn shift_from_guard_and_entries() {
        let guard = 10;
        let entries: &[GuardLogEntry] = &[
            "[1518-11-01 00:05] falls asleep".parse().unwrap(),
            "[1518-11-01 00:25] wakes up".parse().unwrap(),
            "[1518-11-01 00:30] falls asleep".parse().unwrap(),
            "[1518-11-01 00:55] wakes up".parse().unwrap(),
        ];

        let shift = Shift::from_guard_and_entries(guard, entries).unwrap();
        assert_eq!(shift.guard, guard);
        assert_eq!(shift.naps.len(), 2);
        assert_eq!(
            &shift.naps[..2],
            &[
                Nap {
                    start: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
                    end: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0),
                },
                Nap {
                    start: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 30, 0),
                    end: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 55, 0),
                },
            ],
        );
    }

    #[test]
    fn parse_guard_log() {
        let log = GuardLog::parse_lines(EXAMPLE_INPUT).unwrap();
        assert_eq!(log.shift_log.len(), 5);
        let first_shift = &log.shift_log[0];
        assert_eq!(first_shift.guard, 10);
        assert_eq!(
            first_shift.naps,
            &[
                Nap {
                    start: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
                    end: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0),
                },
                Nap {
                    start: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 30, 0),
                    end: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 55, 0),
                }
            ],
        )
    }

    #[test]
    fn nap_duration() {
        let nap = Nap {
            start: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
            end: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0),
        };

        assert_eq!(nap.duration().as_secs(), 20 * 60);
    }
}
