# Api maps directly to the file system
* V#  maps to the directory structure on the local machine

# Files in a given directory are used as the response body based on a few simple rules
* A single file in a directory will be the response for any HTTP method called at its parent directory's uri
* Specific HTTP methods are specified by file name
  * get.json
  * post.json
  * get_index.html
  * post_info.txt
* Any file whose name is an HTTP method will be used for that method
* Anything prefixed with the method name and an underscore will be used for that method
* If there are multiple files available given the previous rules, the response will rotate through all of them. The order will always be the same when the list repeats
* Mixing file rules also works
  * If you have get.json and another file without a specified method called my_file.json, get requests will use get.json while everything else will use my_file.json

# Building responses
* Pipe in and save HTTP requests from curl or something
  * Response is saved in the path of the request
* Load responses from a file
* Read list of requests from a file with some kind of simple syntax
  * GET | https://google.com
  * Maybe this can be piped in too