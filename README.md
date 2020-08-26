Counterfeit is a tool for simulating a REST API. API endpoints map directly to your file system and request bodies are built based on a few simple rules. It's particularly useful for returning JSON responses as they can be edited in your favorite text editor any time you need the data to change. The next time you call the endpoint, you'll get the updated data.

# The Rules
* HTTP methods are specified by file name
  * Any file whose name is an HTTP method will be used for that method
  * Anything prefixed with the method name and an underscore will be used for that method
* Examples
  * `get.json`
  * `post.json`
  * `get_index.html`
  * `post_info.txt`
* If there are multiple files available, the response will rotate through all of them. The order will always be the same when the list repeats
  * `get_first.json`
  * `get_second.json`
  * `get_third.json`
* Any directory prefixed *and* postfixed with an underscore will function as a path variable. Any text can be substituted for this component of the path
  * `/user/_userId_/profile`

# Features In-Progress

## Building responses
* Pipe in and save HTTP requests from curl or something
  * Response is saved in the path of the request
* Load responses from a file
* Read list of requests from a file with some kind of simple syntax
  * GET | https://google.com
  * Maybe this can be piped in too
