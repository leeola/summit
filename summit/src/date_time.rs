use chrono::{FixedOffset, Utc};
use sailfish::{
    runtime::{Buffer, Render},
    RenderError,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<Utc>);
impl DateTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
    pub fn to_local(self, tz: TimeZone) -> LocalDateTime {
        LocalDateTime(self, tz)
    }
}

// TODO: Support named zones and related features, for named selection. chrono_tz?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeZone(FixedOffset);
impl Default for TimeZone {
    fn default() -> Self {
        // NOTE: Defaulting to west coast US for testing purposes, mostly.
        // Long term this should be configurable for communities which have a natural time zone.
        Self(FixedOffset::west_opt(7 * 3600).expect("7*3600 is within max seconds"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalDateTime(DateTime, TimeZone);
impl Render for LocalDateTime {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        let &Self(DateTime(date_time_utc), TimeZone(offset)) = self;
        let date_time = date_time_utc + offset;
        // NIT: Is there a way we can render this without allocating? Seems every component of
        // the formatted date time is a fixed set of characters, no need to alloc just to push a str
        // right? :thinking:
        //
        // For now just using the native formatting for ease. No need to shave this yak.
        //
        // TODO: Time display preferences would be nice.
        b.push_str(&format!("{}", date_time.format("%b %m, %Y %l:%M%P")));
        Ok(())
    }
}
