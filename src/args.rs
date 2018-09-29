use clap::{Arg, App, ArgMatches};


pub fn match_arguments() -> ArgMatches<'static> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        // arguments
        .arg(Arg::with_name("ENGINE")
            .help("Search engine to use")
            .index(1)
            .required(true))
        .arg(Arg::with_name("QUERY")
             .help("Query for search engine")
             .index(2)
             .multiple(true)
             .required(false))

        .get_matches();
    
    matches
}
