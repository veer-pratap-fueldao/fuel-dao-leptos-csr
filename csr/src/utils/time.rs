
use chrono::DateTime;
use uts2ts::uts2ts;
use web_time::{Duration, SystemTime};

pub fn current_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

/// Get day & month -> DD MMM format
/// where DD -> 2 digits
/// MMM -> String representing the month. i.e AUG for august
pub fn get_day_month_time(epoch_secs: u64) -> String {
    let ts = uts2ts(epoch_secs as i64);
    let year = ts.year;

   

    let month = match ts.month {
        1 => "JAN",
        2 => "FEB",
        3 => "MARCH",
        4 => "APRIL",
        5 => "MAY",
        6 => "JUNE",
        7 => "JULY",
        8 => "AUG",
        9 => "SEPT",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => unreachable!(),
    };
    let f = format!("{month} {:02}, {year}  {}:{}", ts.day, ts.hour, ts.second);

    DateTime::from_timestamp(epoch_secs as i64, 0).map_or( f ,|f| format!("{}", f.format("%b %d, %Y %I:%M %p")))

}

pub fn to_hh_mm_ss(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hh = secs / 3600;
    let mm = (secs - hh * 3600) / 60;
    let ss = (secs - hh * 3600) - mm * 60;

    format!("{hh:02}:{mm:02}:{ss:02}")
}

pub async fn sleep(duration: Duration) {
    #[cfg(feature = "ssr")]
    {
        use tokio::time;
        time::sleep(duration).await;
    }
    #[cfg(feature = "hydrate")]
    {
        use gloo::timers::future::sleep;
        sleep(duration).await;
    }
}
