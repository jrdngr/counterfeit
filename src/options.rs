use structopt::StructOpt;

/// Counterfeit is a tool for simulating a REST API. 
/// API endpoints map directly to your file system and request bodies are built based on a few simple rules. 
/// It's particularly useful for returning JSON responses as they can be edited in your favorite text editor 
/// any time you need the data to change. The next time you call the endpoint, you'll get the updated data.
#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
pub struct CounterfeitOptions {
    /// Unimplemented --
    /// Paths will match if they have the same number of components. 
    /// The response will be the path with the greatest number of matching components.
    #[structopt(short, long)]
    lenient: bool,

    /// Unimplemented --
    /// Write POST requests to their request path overwriting post files that are already there
    #[structopt(short, long)]
    write_post: bool,

    /// Unimplemented --
    /// If a post.json exists, use it as a base and apply POST request as a diff.
    /// If no post.json exists, write the response to post.json.
    #[structopt(short, long)]
    diff_post: bool,

    /// Unimplemented --
    /// Sets the directory name template for Path parameters
    /// Use double brackets around the parameter.
    /// Example: "_{{}}_" -> ../_someIdentifier_/..
    #[structopt(short, long)]
    param_template: Option<String>,

    #[structopt(subcommand)]
    subcommand: CounterfeitSubcommand,
}

#[derive(StructOpt, Debug)]
pub enum CounterfeitSubcommand {
    /// Unimplemented --
    /// Pipe in an HTTP response to save the body to the path given in the original request
    /// The file name will be based on the HTTP method of the request.
    /// If there's already a file for that method in the directory, all matching files would be numbered.
    /// You're free to rename all of those files
    #[structopt(name = "save")]
    Save {
        response: String,
        
        /// If a file already exists in the target directory, it will be overwritten
        #[structopt(short, long)]
        overwrite: bool,
    },
} 