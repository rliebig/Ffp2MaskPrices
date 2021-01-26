use select::predicate::Class;

use regex::Regex;

use std::env;
use std::path::Path;
use std::io::{Write, Error, BufRead};
use std::fs::OpenOptions;

use clap::{Arg, App, SubCommand};

static REGEX_STRING: &str = r"\d+((x)|(er)|(St)|( )(Stück|Stk|STK))";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_test() {
        let re = Regex::new(REGEX_STRING).unwrap();
        assert!(re.is_match("50x"));
        assert!(re.is_match("5x"));
        assert!(re.is_match("50 Stück"));
        assert!(re.is_match("5 Stk"));
        assert!(re.is_match("10er"));
        assert!(re.is_match("50St"));
        assert!(re.is_match("(20er)"));

        assert!(!re.is_match("5 lagig"));
        assert!(!re.is_match("EN149:2001+A1:2009"));
        assert!(!re.is_match("FFP2 "));
        assert!(!re.is_match("Einweg-3-Lagen-Schutzmaske"));
    }
}


fn save_average(avg : f32) {
    let path = Path::new("avg.txt");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&path);
    let mut file = match file {
        Err(why) => panic!("Couldn't create:{}", why),
        Ok(file) => file,
    };

    let now = get_date_string();
    let result = file.write_fmt(format_args!("{} {}\n", now, avg));
    match result {
        Ok(_) => {}
        Err(why) => {panic!("Could not write to file: {}", why);}
    }
}

fn get_date_string() -> String {
    let now = chrono::Utc::now();
    let now = now.to_rfc3339();
    return now;
}

fn scrap_today_data(path_arg : & str) -> Result<(), reqwest::Error>{
    let url = "https://www.amazon.de/s?k=ffp2+masken&__mk_de_DE=%C3%85M%C3%85%C5%BD%C3%95%C3%91&ref=nb_sb_noss_1";
    println!("Downloading document");
    let req = reqwest::blocking::get(url)?
        .text()?;

    let document = select::document::Document::from(req.as_str());
    println!("Document created.");
    let mut headings = vec![];
    let mut prices = vec![];

    for node in document.find(Class("a-size-base-plus")).take(20) {
        headings.push(node.text());
    }

    for node in document.find(Class("a-price")).take(20) {
        prices.push(node.text());
    }

    let mut collected_price : f32 = 0.0;
    let mut item : f32 = 0.0;
    let mut conv_prives = vec![];
    for (headline, price) in headings
        .iter_mut()
        .zip(prices.iter_mut()) {
        if !headline.contains("FFP2") {
            continue;
        }
        let price  = price.split_whitespace().next().unwrap();
        let price_float : f32 = price.replace(",", ".").parse().unwrap();

        let re = Regex::new(REGEX_STRING).unwrap();
        println!("{}", headline);
        let text  = re.find(headline);
        if text == None {
            continue;
        }
        let text = text.unwrap();
        let text = &headline[text.start()..text.end()];

        println!("substr: {}", text);
        let re = Regex::new(r"\d*").unwrap();
        let text = re.captures(text).unwrap();
        let actual_number = text.get(0).map_or("1", |m| m.as_str()).replace("x", "");
        println!("actual_number: {}", actual_number);
        let converted_number : i32 = actual_number.parse().unwrap();
        println!("stk: {}", converted_number);

        println!("{:?}: {}", price, headline);
        conv_prives.push(price_float / (converted_number as f32));
        println!("calculated per piece value: {}", price_float / (converted_number as f32));
        collected_price += price_float / (converted_number as f32);
        item = item + 1.0;
    }
    println!("Average: {}", collected_price / item);

    let mut owned_string : String = "".to_owned();
    let date_string : &str = &get_date_string();
    let date_string = date_string
        .replace("+", "")
        .replace(":", "")
        .replace(".", "");
    let final_string : &str = ".txt";
    owned_string.push_str(path_arg);
    owned_string.push_str("data");
    owned_string.push_str(date_string.as_str());
    owned_string.push_str(final_string);
    let path = Path::new(owned_string.as_str());
    println!("{}", owned_string.as_str());

    let mut file = match std::fs::File::create(&path) {
        Err(why) => panic!("Couldn't create:{}", why),
        Ok(file) => file,
    };

    for i in 0..conv_prives.len() {
        file.write_fmt(format_args!("{} {}\n", i, conv_prives[i]));
    }
    save_average(collected_price / item);

    Ok(())
}

fn recalculate_avg() {
    let current_dir = std::env::current_dir();
    let current_dir = match current_dir {
        Ok(PathBuf) => PathBuf,
        Err(why) => panic!("not possible to traverse directory: {}", why),
    };

    let iterator = std::fs::read_dir(current_dir);
    let iterator = match iterator {
        Ok(iterator) => iterator,
        Err(why) => panic!("not possible to traverse directory: {}", why),
    };

    let avg : Vec<(&str, f32)> = vec![];
    for entry in iterator {
        let entry = entry;
        let entry = match entry {
            Ok(entry) => entry,
            Err(why) => panic!("not possible to traverse directory: {}", why),
        };
        let path = entry.path();
        println!("{}", path.as_path().display());
        let filename = path.as_path().file_name();
        let filename = match filename {
            Some(filename) => filename,
            None => panic!("not possible to traverse directory"),
        };

        let name = filename.to_str();
        let name = match name {
            Some(name) => name,
            None => panic!("not possible to traverse directory"),
        };
        println!("{}", name);

        if name.starts_with("data") && name.ends_with(".txt") {
            println!("work on it.");
            let actual_path = path.as_path();

            let actual_file = OpenOptions::new()
                .read(true)
                .open(&actual_path);

            let actual_file = match actual_file {
                Ok(val) => val,
                Err(why) => panic!("Could not open file: {}", why),
            };

            let lines = std::io::BufReader::new(actual_file).lines();
            let mut current_avg : f32  = 0.0;
            let mut amount : f32 = 0.0;
            for line in lines {
                if let Ok(ip) = line {
                    let first_part = ip.split_whitespace()
                        .next()
                        .unwrap();
                    let float : f32 = first_part.parse().unwrap();

                    current_avg += float;
                    amount += 1.0;
                }
            }
            current_avg = current_avg / amount;
            println!("{}", current_avg);
        }
    }


    let avg_path = Path::new("avg.txt");
    let avg_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(&avg_path);

    let mut file = match avg_file {
        Err(why) => panic!("Couldn't modify: {}", why),
        Ok(file) => file
    };
}

fn main() -> Result<(), reqwest::Error> {
    //todo: interpret argument for file names
    let matches = App::new("Ffp2MaskPrices")
        .version("0.1")
        .author("Richard Liebig <liebig.richard >AT< hotmail.com>")
        .about("Scraps amazon prices for FFF masks in german language")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Set directory for data saving")
            .default_value(""))
        .subcommand(SubCommand::with_name("scrap")
            .about("scrap current prices"))
        .subcommand(SubCommand::with_name("recalculate-avg")
            .about("recalculate averages"))
        .subcommand(SubCommand::with_name("serve")
            .about("produces png plots for all data and serves them via a webserver"))
        .get_matches();

    let path = matches.value_of("path").unwrap_or("");
    println!("{}", path);
    if let Some(matches) = matches.subcommand_matches("scrap") {
        scrap_today_data(path);
    } else if let Some(matches) = matches.subcommand_matches("recalculate-avg") {
       recalculate_avg();
    } else {
        scrap_today_data("./");
    }

    Ok(())
}
