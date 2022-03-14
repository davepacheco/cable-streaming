use anyhow::Context;
use cable_streaming::xmltv::{Channel, Program, Tv};
use chrono::Datelike;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), anyhow::Error> {
    let filename = "local-data/xmltv-listings-2022-03-13.out";
    let file_reader =
        File::open(filename).with_context(|| format!("open {:?}", filename))?;
    let mut buf_reader = BufReader::new(file_reader);

    // Skip HTTP headers
    loop {
        let mut line = String::new();
        buf_reader
            .read_line(&mut line)
            .with_context(|| format!("read {:?}", filename))?;
        if line.len() == 2 && line == "\r\n" {
            break;
        }
    }

    eprintln!("parsing ... ");
    let tv = cable_streaming::xmltv::Tv::from_reader(buf_reader)
        .with_context(|| format!("parsing XMLTV file {:?}", filename))?;

    // TODO make a struct, maybe even configurable
    let filters = vec![
        "AMC-PHD", "BBC", "BRAVO", "FX", "FXX-HD-W", "TBS", "TNT", "USAHD-P",
        "VH1",
    ];

    let all_channels: BTreeMap<&str, &Channel> =
        tv.channels.iter().map(|c| (c.id.as_str(), c)).collect();

    // TODO should just be a BTreeSet
    let selected_channels: BTreeMap<&str, &Channel> = tv
        .channels
        .iter()
        .filter_map(|c| {
            filters
                .iter()
                .any(|f| *f == c.callsign())
                .then(|| (c.id.as_str(), c))
        })
        .collect();
    println!("selected channels:");
    for (_, c) in &selected_channels {
        print_channel(c);
    }

    // Filter movies based on basic criteria.  Dedup them into "found_movies".
    let mut found_movies: BTreeMap<&str, Vec<&Program>> = BTreeMap::new();
    for p in &tv.programs {
        if !selected_channels.contains_key(p.channel.as_str()) {
            continue;
        }

        if p.category != "Movie" {
            continue;
        }

        found_movies
            .entry(p.title.as_str())
            .or_insert_with(|| Vec::new())
            .push(&p);
    }

    // Filter out movies based on not having any showtimes during our preferred
    // range.
    let found_movies: BTreeMap<&str, Vec<&Program>> = found_movies
        .into_iter()
        .filter(|(_, showtimes)| {
            showtimes.iter().any(|p| {
                matches!(
                    p.start.weekday(),
                    chrono::Weekday::Sat | chrono::Weekday::Sun
                )
            })
        })
        .collect();

    println!("movies found on these channels on weekends:");
    println!("{:40} #SHOW #CH EXAMPLE", "");
    for (title, programs) in found_movies.iter() {
        let mut showings_by_channel: BTreeMap<&str, u16> = BTreeMap::new();
        let mut total = 0;

        for p in programs {
            let c: &mut u16 =
                showings_by_channel.entry(p.channel.as_str()).or_insert(0);
            *c = *c + 1;
            total = total + 1;
        }

        let max_entry =
            showings_by_channel.iter().max_by_key(|(_, count)| *count).unwrap();
        let max_channel = all_channels.get(max_entry.0).unwrap(); // XXX

        println!(
            "{:60} {:5} {:3} {:8} ({:2})",
            title,
            total,
            showings_by_channel.len(),
            max_channel.callsign(),
            max_entry.1,
        );
    }

    Ok(())
}

fn print_all_channels(tv: &Tv) {
    println!("channels:");
    for c in &tv.channels {
        print_channel(&c);
    }
}

fn print_channel(c: &Channel) {
    println!("{:5} {:10} {}", c.channel_number(), c.callsign(), c.full_name());
}
