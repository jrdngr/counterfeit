use counterfeit_core::CounterfeitRunConfig;
use structopt::StructOpt;

/// Counterfeit is a tool for simulating a REST API.
/// API endpoints map directly to your file system and request bodies are built based on a few simple rules.
/// It's particularly useful for returning JSON responses as they can be edited in your favorite text editor
/// any time you need the data to change. The next time you call the endpoint, you'll get the updated data.
#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
pub enum CounterfeitOptions {
    /// Runs the counterfeit server
    #[structopt(name = "run")]
    Run(CounterfeitRunOptions),

    /// Unimplemented --
    /// Pipe in an HTTP response to save the body to the path given in the original request
    /// The file name will be based on the HTTP method of the request.
    /// If there's already a file for that method in the directory, all matching files would be numbered.
    /// You're free to rename all of those files
    #[structopt(name = "save")]
    Save(CounterfeitSaveOptions),
}

#[derive(StructOpt, Debug)]
pub struct CounterfeitRunOptions {
    /// Sets the base directory to serve responses from
    #[structopt(short = "b", long, default_value = "./responses")]
    pub base_path: String,

    /// Unimplemented --
    /// Writes requests to disk depending on the HTTP method.
    /// POST will add a new GET file and renumber all GET files.
    /// PUT will add a new GET file and delete any existing GET files.
    /// DELETE will delete all matching GET files.
    /// PATCH will diff all matching GET files.
    #[structopt(short = "w", long)]
    pub write: bool,

    /// Unimplemented --
    /// If a request is received that has no matching response, an empty
    /// file will be created with the name of the method and a .json extension
    #[structopt(short, long)]
    pub create_missing: bool,

    /// Silences printing request and response info to the console
    #[structopt(short = "s", long)]
    pub silent: bool,

    /// Sets the host of the local server
    #[structopt(short = "h", long, default_value = "127.0.0.1")]
    pub host: String,

    /// Sets the port of the local server
    #[structopt(short = "p", long, default_value = "3000")]
    pub port: u16,

    /// Sets the directory prefix for path parameters.
    /// Example: "_" -> ../_anyIdentifier/..
    #[structopt(long = "prefix", default_value = "_")]
    pub param_prefix: String,

    /// Sets the directory postfix for path parameters.
    /// Example: "_" -> ../anyIdentifier_/..
    #[structopt(long = "postfix", default_value = "_")]
    pub param_postfix: String,

    /// Sets the directory prefix and postfix for path parameters.
    /// Example: "_" -> ../_anyIdentifier_/..
    #[structopt(
        long = "surround",
        conflicts_with_all = &["param_prefix", "param_postfix"]
    )]
    pub param_surround: Option<String>,

    /// Unimplemented --
    /// Server will ignore any file with this prefix
    #[structopt(long = "ignore")]
    pub ignore_file_prefix: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct CounterfeitSaveOptions {
    pub response: String,

    /// Sets the base directory to store responses to
    #[structopt(short = "b", long, default_value = "./responses")]
    pub base_path: String,

    /// If a file already exists in the target directory, it will be overwritten
    #[structopt(short = "o", long)]
    pub overwrite: bool,
}

impl From<CounterfeitRunOptions> for CounterfeitRunConfig {
    fn from(options: CounterfeitRunOptions) -> Self {
        let CounterfeitRunOptions {
            base_path,
            write,
            create_missing,
            silent,
            host,
            port,
            param_prefix,
            param_postfix,
            param_surround,
            ..
        } = options;

        let socket = format!("{}:{}", host, port)
            .parse()
            .expect("Invalid socket address");

        let (prefix, postfix) = if let Some(surround) = param_surround {
            (surround.clone(), surround)
        } else {
            (param_prefix, param_postfix)
        };

        Self {
            base_path,
            write,
            create_missing,
            silent,
            socket,
            prefix,
            postfix,
        }
    }
}
