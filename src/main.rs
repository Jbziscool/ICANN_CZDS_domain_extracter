use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use rustc_hash::FxHashSet as HashSet;
use std::time::Instant;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        let exe_name = env::args()
            .next()
            .and_then(|p| Path::new(&p).file_name().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "ohjelma".to_string());

        eprintln!("väärä käyttö");
        eprintln!("Käyttö: {} <tld> <tiedosto>", exe_name);
        eprintln!("Esim: {} shop shop.txt", exe_name);
        eprintln!("tää esimerkki yrittää nappaa kaikki .shop päätteiset domainit tiedostosta shop.txt ja sit kirjottaa ne takas siihen samaan tiedostoon.");
        std::process::exit(1);
    }

    let tld = &args[1];
    let input_path = &args[2];

    let pattern = format!(r"\b[^\s]+\.{}\b", regex::escape(tld));
    let re = Regex::new(&pattern).expect("sä sait mun regexin vituiks");

    let file = File::open(input_path)?;
    let reader = BufReader::with_capacity(1_048_576, file);

    let mut domains = HashSet::default();
    let mut line_count = 0;
    let start_time = Instant::now();

    for line in reader.lines() {
        let line = line?;
        line_count += 1;

        if line_count % 1_000_000 == 0 {
            println!("Luettu jo {} riviä ({:.2?})", line_count, start_time.elapsed());
        }

        for m in re.find_iter(&line) {
            domains.insert(m.as_str().to_owned());
        }
    }

    // --- kirjotetaan tulokset takas samaan tiedostoon ---
    let mut list: Vec<_> = domains.into_iter().collect();
    list.sort();

    let mut out = BufWriter::new(File::create(input_path)?);
    for d in &list {
        writeln!(out, "{}", d)?;
    }

    println!(
        "✅ Valmis! Löyty {} uniikkia .{} domainii tiedostosta {} ({:.2?})",
        list.len(),
        tld,
        input_path,
        start_time.elapsed()
    );

    Ok(())
}
