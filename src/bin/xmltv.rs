use anyhow::Context;
use cable_streaming::xmltv::{Tv, Channel};
use std::{
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

    let channels: Vec<&Channel> = tv
        .channels
        .iter()
        .filter(|c| filters.iter().any(|f| *f == c.callsign()))
        .collect();
    println!("selected channels:");
    for c in &channels {
        print_channel(c);
    }

    println!("movies found on these channels:");
    for p in &tv.programs {
        if !channels.iter().any(|c| c.id == p.channel) {
            continue;
        }

        if p.category != "Movie" {
            continue;
        }

        println!("    {}", p.title);
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
