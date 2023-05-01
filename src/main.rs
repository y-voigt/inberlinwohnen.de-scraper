use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::{blocking, Error};
use scraper::{Html, Selector};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut next_run: Instant = Instant::now();

    let current_time = || chrono::Local::now().time();

    let url: &str = "https://inberlinwohnen.de/wohnungsfinder/";
    let email_from: &str = "y.voigt@outlook.com";
    let email_to = email_from;
    // let subject: String = format!("New listings at: {:?}", current_time);
    let mut newest_listing: String = String::new();

    let sleep_time: f32 = 300.0;

    loop {
        if Instant::now() >= next_run {
            // let current_time: chrono::NaiveTime = chrono::Local::now().time();

            let response: Result<String, Error> = throttled_request(url);

            let result: String = parse_page(response.unwrap());

            if newest_listing.is_empty() {
                newest_listing = result;
            } else if result == newest_listing {
                println!("{:?}: {}", current_time(), "No new listing!");
            } else {
                newest_listing = result;
                println!("{:?}: {}", current_time(), newest_listing);
            }

            // accurate 5 minute wait time even in the case of disruptions
            next_run += Duration::from_secs(sleep_time as u64);
        }
        thread::sleep(Duration::from_secs((sleep_time * 0.75) as u64));
    }
}

fn throttled_request(url: &str) -> Result<String, Error> {
    let response: blocking::Response = blocking::get(url)?;
    return response.text();
}

fn parse_page(response: String) -> String {
    let document: Html = Html::parse_document(&response);
    let actual_listings_sel: Selector =
        Selector::parse("div.result-list li.tb-merkflat.ipg:first-child").unwrap();
    let listings = document.select(&actual_listings_sel).next().unwrap();
    return listings.value().attr("id").unwrap().to_string();
}

/* fn build_mail() {
    let email = Message::builder()
        .to("hello@example.com")
        .from("me@hello.com")
        .subject("Example subject")
        .text("Hello, world!")
        .build()
        .unwrap();
} */
