use chrono::NaiveDateTime;
use serde::Serializer;

pub fn sent_at_date_format<S>(
    date: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format("%H:%M"));
    serializer.serialize_str(&s)
}
