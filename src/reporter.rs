use std::time::Duration;

use colored::*;

use crate::config::Config;
use serde::{Deserialize, Serialize};

const COLORED_DURATION_PERCENT_THRESHOLD: f64 = 5.0;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Results {
    pub config: Config,
    // The indices of the first vec correspond to the indices of the config.requests
    pub times: Vec<Vec<Duration>>,
    pub total: Duration,
}

fn fmt_duration(d: &Duration) -> String {
    format!("{:.1}ms", d.as_micros() as f64 / 1000.0)
}

fn fmt_duration_difference(d1: &Duration, d2: &Duration) -> ColoredString {
    let d1ms = d1.as_micros() as f64 / 1000.0;
    let d2ms = d2.as_micros() as f64 / 1000.0;
    let difference = d2ms - d1ms;
    let difference_percent = difference / d1ms * 100.0;
    let threshold = COLORED_DURATION_PERCENT_THRESHOLD;
    match difference_percent {
        d if d < -threshold => format!("-{:.0}%", f64::abs(d)).green(),
        d if d >= threshold => format!("+{:.0}%", f64::abs(d)).red(),
        d if d < -1.0 => format!("-{:.0}%", f64::abs(d)).white(),
        d if d >= 1.0 => format!("+{:.0}%", f64::abs(d)).white(),
        _ => "=".white(),
    }
}

fn get_duration_percentile(d: &[Duration], percentile: u8) -> &Duration {
    &d[(d.len() * (percentile as usize) / 100) - 1]
}

fn print_duration_percentile(d: &[Duration], percentile: u8) {
    println!(
        "\t{}%: {}",
        percentile,
        fmt_duration(get_duration_percentile(d, percentile))
    );
}

fn print_duration_percentile_with_baseline(d: &[Duration], bd: &[Duration], percentile: u8) {
    println!(
        "\t{}%: {} ({})",
        percentile,
        fmt_duration(get_duration_percentile(d, percentile)),
        fmt_duration_difference(
            get_duration_percentile(bd, percentile),
            get_duration_percentile(d, percentile)
        ),
    );
}

pub fn report_results(current: &Results, baseline: &Option<Results>) {
    println!("Total time elapsed: {:.2}s", current.total.as_secs_f32());
    match baseline {
        Some(b) => report_results_with_baseline(current, b),
        None => report_results_without_baseline(current),
    }
}

pub fn report_results_without_baseline(current: &Results) {
    for (i, rc) in current.config.requests.iter().enumerate() {
        let mut d = current
            .times
            .get(i)
            .expect("the vec of times should have the same length as the vec of requests")
            .clone();
        d.sort_unstable();
        println!("{}", rc.url);
        print_duration_percentile(&d, 50);
        print_duration_percentile(&d, 90);
        print_duration_percentile(&d, 95);
        print_duration_percentile(&d, 99);
    }
}

pub fn report_results_with_baseline(current: &Results, baseline: &Results) {
    if current.config != baseline.config {
        println!("Can't use baseline, as config has changed");
        std::process::exit(1)
    }

    for (i, rc) in current.config.requests.iter().enumerate() {
        let mut dc = current
            .times
            .get(i)
            .expect("the vec of times should have the same length as the vec of requests")
            .clone();
        dc.sort_unstable();

        let mut db = baseline
            .times
            .get(i)
            .expect("the vec of times should have the same length as the vec of requests")
            .clone();
        db.sort_unstable();

        println!("{}", rc.url);

        print_duration_percentile_with_baseline(&dc, &db, 50);
        print_duration_percentile_with_baseline(&dc, &db, 90);
        print_duration_percentile_with_baseline(&dc, &db, 95);
        print_duration_percentile_with_baseline(&dc, &db, 99);
    }
}
