use std::io;

use anyhow::Result;

pub fn run() -> Result<u8> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = io::BufReader::new(stdin);
    let mut total = iowrap::Pos::new(stdin);
    let mut compressed = flate2::read::DeflateEncoder::new(&mut total, flate2::Compression::fast());
    let compressed = io::copy(&mut compressed, &mut iowrap::Ignore::new())?;
    let total = total.position();
    println!("{} -> {} ({:.2}%)", human_size(total), human_size(compressed), 100.* (compressed as f64) / (total as f64));
    Ok(0)
}

fn human_size(bytes: u64) -> String {
    if bytes < 900 {
        return format!("{}", bytes);
    }

    if bytes > 2u64.pow(53) {
        return format!("~{}EB", bytes / 2u64.pow(50));
    }

    let mut bytes = bytes as f64;

    bytes /= 1024.;

    for suffix in &["k", "M", "G", "T"] {
        if bytes < 900. {
            return format!("{:.2}{}B", bytes, suffix);
        }

        bytes /= 1024.0;
    }

    format!("{:.2}PB", bytes)
}

#[test]
fn human_size_test() {
    assert_eq!("0", human_size(0));
    assert_eq!("1", human_size(1));
    assert_eq!("899", human_size(899));
    assert_eq!("0.88kB", human_size(900));
    assert_eq!("0.88kB", human_size(901));
    assert_eq!("0.98kB", human_size(1000));
    assert_eq!("1.00kB", human_size(1024));
    assert_eq!("1.07kB", human_size(1100));
    assert_eq!("1.76kB", human_size(1800));
    assert_eq!("18.00MB", human_size(18 * 1024u64.pow(2)));
    assert_eq!("18.00GB", human_size(18 * 1024u64.pow(3)));
    assert_eq!("18.00TB", human_size(18 * 1024u64.pow(4)));
    assert_eq!("~18EB", human_size(18 * 1024u64.pow(5)));
    assert_eq!("~16383EB", human_size(16383 * 1024u64.pow(5)));
    assert_eq!("~16383EB", human_size(16383 * 1024u64.pow(5) + 999 * 1024u64.pow(4)));
}
