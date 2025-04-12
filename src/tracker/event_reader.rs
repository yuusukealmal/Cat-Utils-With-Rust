use std::io::Cursor;

use chrono::{Local, NaiveDateTime};

use crate::config::structs::EventData;

pub fn get_event_lists(events: String) -> Result<Vec<EventData>, Box<dyn std::error::Error>> {
    let mut current_events = vec![];

    let event_list: Vec<Vec<String>> = events
        .lines()
        .skip(1)
        .take(events.lines().count() - 2)
        .map(|line| {
            line.split('\t')
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    for event in event_list {
        let start_date = &event[0];
        let start_time = if &event[1] == "0" { "0000" } else { &event[1] };
        let end_date = &event[2];
        let end_time = if &event[3] == "0" { "0000" } else { &event[3] };

        let start_str = format!("{}{}", start_date, start_time);
        let end_str = format!("{}{}", end_date, end_time);
        let start = NaiveDateTime::parse_from_str(&start_str, "%Y%m%d%H%M")?;
        let end = NaiveDateTime::parse_from_str(&end_str, "%Y%m%d%H%M")?;
        let now = Local::now().naive_local();

        if now < end && event.len() > 25 && event[8] == "1" {
            let length: u32 = event[9].parse()?;

            let gatya_id: u32 = event[10 + (15 * (length - 1)) as usize].parse()?;
            let rare_chance: u32 = event[16 + (15 * (length - 1)) as usize].parse()?;
            let super_rare_chance: u32 = event[18 + (15 * (length - 1)) as usize].parse()?;
            let uber_rare_chance: u32 = event[20 + (15 * (length - 1)) as usize].parse()?;
            let legend_rare_chance: u32 = event[22 + (15 * (length - 1)) as usize].parse()?;
            let banner_text = event[24 + (15 * (length - 1)) as usize].clone();
            let bp = [
                "白金",
                "傳說",
                "プラチナガチャ",
                "レジェンドガチャ",
                "PLATINUM",
                "Legend",
                "플래티넘",
                "레전드",
            ];

            if bp.iter().any(|key| banner_text.contains(key)) {
                continue;
            }

            let force = if event[20 + 1 + (15 * (length - 1)) as usize].as_str() == "1" {
                true
            } else {
                false
            };

            let event_struct = EventData {
                cc: None,
                id: gatya_id,
                rare: rare_chance,
                super_rare: super_rare_chance,
                uber_rare: uber_rare_chance,
                legend: legend_rare_chance,
                banner_text,
                force: force,
                gatya_data: None,
                unit_buy: None,
            };

            current_events.push(event_struct);
        }
    }

    Ok(current_events)
}
