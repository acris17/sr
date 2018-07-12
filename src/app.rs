use ::args;
use ::engine;


pub fn start() {
    let mut app = Application::default();
    app.setup();
    app.run();
}


#[derive(Debug, Default)]
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
        if self.engine == "google" {
            engine::google(&self.query);
        } 
        else if self.engine == "wiki" {
            engine::wikipedia(&self.query);
        }
        else if self.engine == "crates" {
            engine::crates_io(&self.query);
        }
        else if self.engine == "rust" {
            engine::rust_docs(&self.query);
        }
        else if self.engine == "youtube" {
            engine::youtube(&self.query);
        }
        else if self.engine == "dict" {
            engine::dict(&self.query);
        }
    }
}
