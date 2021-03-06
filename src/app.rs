use ::args;
use ::commands;


pub fn start() {
    let mut app = Application::default();
    app.setup();
    app.run();
}


#[derive(Default)]
struct Application {
    engine: String,
    query:  Vec<String>,
}
impl Application {
    fn default() -> Application {
        Application {..Default::default()}
    }
    fn setup(&mut self) {
        let matches = args::match_arguments();

        // arguments
        if let Some(engine) = matches.value_of("ENGINE") {
            self.engine = String::from(engine);
        }
        if let Some(query) = matches.values_of("QUERY") {
            self.query = query.map(String::from).collect();
        }
    }
    fn run(&self) {
        if self.engine == "go" {
            commands::search_google(&self.query);
        } 
        else if self.engine == "wiki" {
            commands::search_wikipedia(&self.query);
        }
        else if self.engine == "crates" {
            commands::search_crates(&self.query);
        }
        else if self.engine == "rust" {
            commands::search_rust(&self.query);
        }
        else if self.engine == "yt" {
            commands::search_youtube(&self.query);
        }
        else if self.engine == "dict" {
            commands::search_dictionary(&self.query);
        }
        else if self.engine == "list" {
            commands::list_engines();
        }
    }
}
