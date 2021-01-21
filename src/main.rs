use select::predicate::Class;
use regex::Regex;
use std::env;
use tui::Terminal;
use tui::layout::{Direction, Layout, Constraint};
use tui::style::{Style, Color};
use tui::widgets::{Borders, BarChart, Block};
use tui::backend::CrosstermBackend;
use std::path::Path;
use std::io::Write;


fn regex_test() {
    let re = Regex::new(r"\d+((x)|(er)|(St)|( )(Stück|Stk))").unwrap();
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

fn save_average(avg : f32) {
    let path = Path::new("avg.txt");
    let mut file = match std::fs::File::create(&path) {
        Err(why) => panic!("Couldn't create:{}", why),
        Ok(file) => file,
    };

    let now = chrono::Utc::now();
    let now = now.to_rfc3339();
    file.write_fmt(format_args!("{} {}", now, avg));
}

fn scrap_today_data() -> Result<(), reqwest::Error>{

    regex_test();
    let url = "https://www.amazon.de/s?k=ffp1+maske&__mk_de_DE=%C3%85M%C3%85%C5%BD%C3%95%C3%91&ref=nb_sb_noss_1";
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
        let price  = price.split_whitespace().next();
        let price_float : f32 = price.unwrap().replace(",", ".").parse().unwrap();

        let re = Regex::new(r"\d+((x)|(er)|(St)|( )(Stück|Stk|STK))").unwrap();
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
        collected_price += price_float / (converted_number as f32);
        item = item + 1.0;
    }
    println!("Average: {}", collected_price / item);

    let path = Path::new("data.txt");
    let mut file = match std::fs::File::create(&path) {
        Err(why) => panic!("Couldn't create:{}", why),
        Ok(file) => file,
    };

    for i in 0..conv_prives.len() {
        file.write_fmt(format_args!("{} {}\n", i, conv_prives[i]));
    }
    save_average(collected_price / item);


    display_data_chart(conv_prives);
    Ok(())
}

fn display_help() {
    println!("ffp2_mask_prices takes these arguments:");
    println!("--scrap : Downloads current market data");
    println!("--display : ")
}

struct App<'a> {
    data: Vec<(&'a str, u64)>,
}

impl<'a>App<'a> {
    fn new() -> App<'a> {
        App {
            data: vec![
            ]
        }
    }

    fn update(&mut self, number : u64) {
        self.data.push(("test", number));
    }
}

fn display_data_chart(conv_data : Vec<f32>) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    for data in conv_data {
          app.update((data * 100.0).round() as u64);
    }

    terminal.clear();
    loop {
        terminal.draw(
            |f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size())
                    ;

                let barchart = BarChart::default()
                    .block(Block::default()
                    .title("Preise in Cent."))
                    .data(&app.data)
                    .bar_width(9)
                    .bar_style(Style::default().fg(Color::Yellow))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
                f.render_widget(barchart, chunks[0]);
            }
        );
    }

    Ok(())
}

fn display_data() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    loop {
        terminal.draw(
            |f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size())
                    ;

                let barchart = BarChart::default()
                    .block(Block::default()
                        .title("Preise in Cent."))
                    .data(&app.data)
                    .bar_width(9)
                    .bar_style(Style::default().fg(Color::Yellow))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
                f.render_widget(barchart, chunks[0]);
            }
        );
    }

    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    //scrap_today_data();

    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        scrap_today_data();
        display_data();
    }
    let main_arg  = &args[1];
    if main_arg == "-h" || main_arg == "--help" {
        display_help();
    } else if main_arg == "--scrap" {
        scrap_today_data();
    } else if main_arg == "--display" {
        display_data();
    }

    Ok(())
}