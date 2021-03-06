extern crate clap;
extern crate reqwest;
extern crate futures;

use reqwest::header::Headers;


#[macro_use] extern crate hyper;
header! { (XEmitEmail, "x-emit-email") => [String] }

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Emit")
        .version("0.1")
        .author("Marek Counts <countsmarek@gmail.com>")
        .about("Command line untility to assist with using the emit.sh service")
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("--email")
                .required(false)
                .short("-e")
                .help("Ends the emited link to the above email")
                .index(2),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

        if let Some(i) = matches.value_of("input") {

            //let mut email_header : Header;
            let mut headers = Headers::new();


            if let Some(e) = matches.value_of("--email") {
                headers.set(XEmitEmail(e.to_owned()));
                println!("Email address to send to: {}", e);
            }

            let form = reqwest::multipart::Form::new()
                .file("file", i).unwrap();

            let mut response = reqwest::Client::new()
                .post("http://emit.sh")
                .headers(headers)
                .multipart(form)
                .send().unwrap();

            println!("{}", response.text().unwrap());
            
        }
}
