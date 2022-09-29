use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DURATION_TIME: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("s", 1);
        m.insert("sec", 1);
        m.insert("second", 1);
        m.insert("seconds", 1);
        m.insert("m", 60);
        m.insert("min", 60);
        m.insert("minute", 60);
        m.insert("minutes", 60);
        m.insert("h", 3600);
        m.insert("hour", 3600);
        m.insert("hours", 3600);
        m.insert("d", 86400);
        m.insert("day", 86400);
        m.insert("days", 86400);
        m.insert("mo", 2630000);
        m.insert("month", 2630000);
        m.insert("months", 2630000);
        m.insert("y", 31536000);
        m.insert("year", 31536000);
        m.insert("years", 31536000);
        m
    };
}