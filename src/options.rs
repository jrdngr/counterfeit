use std::net::SocketAddr;

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
    #[structopt(short = "l", long)]
    pub lenient: bool,

    /// Unimplemented --
    /// Writes requests to disk depending on the HTTP method.
    /// POST will add a new GET file and renumber all GET files.
    /// PUT will add a new GET file and delete any existing GET files.
    /// DELETE will delete all matching GET files.
    /// PATCH will dif all matching GET files.
    #[structopt(short = "w", long)]
    pub write: bool,

    /// Sets the port of the local server
    #[structopt(short = "p", long)]
    pub port: Option<u16>,

    /// Sets the socket address of the local server
    #[structopt(short = "s", long, default_value = "127.0.0.1:3000")]
    pub socket: SocketAddr,

    /// Sets the directory prefix for path parameters.
    /// Example: "_" -> ../_anyIdentifier/..
    #[structopt(long = "prefix")]
    pub param_prefix: Option<String>,

    /// Sets the directory postfix for path parameters.
    /// Example: "_" -> ../anyIdentifier_/..
    #[structopt(long = "postfix")]
    pub param_postfix: Option<String>,

    /// Sets the directory prefix and postfix for path parameters.
    /// Example: "_" -> ../_anyIdentifier_/..
    /// 
    /// If used with --prefix or --postfix, the given symbol will surround the other prefix/postfix
    /// Example: "--prefix { --postfix } --surround _" -> ../_{anyIdentifier}_/..
    #[structopt(long = "surround")]
    pub params_surround: Option<String>,

    #[structopt(subcommand)]
    pub subcommand: Option<CounterfeitSubcommand>,
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
        #[structopt(short = "o", long)]
        overwrite: bool,
    },
}
