# Hermes

Hermes is a light-weight API client in the terminal with VIM keymaps.

## How it works

Hermes API routes will be grouped into collections. Each collection can have individual requests.

Each collection can have different sets of environments they can run on. An environment is defined
in a `.hermes` file of type `collection`. See below for an example.

A request include the method, url, headers, body, etc... Basically anything that a normal API route would have.
You can define a route within Hermes or create a file within the collection folder with the extension `.hermes`. The file
its just a normal text file.

Here is an example of a route:

```
# request files must have .hermes extesion and must not begin with '.'
metadata {
    type request
    name "Greet Hermes"
}

request {
    method post
    url https://{{HOST}}:8000/greet
    body json
    headers H
    variables V
    queries Q
}

headers::H {
        Content-Type 1 application/json
        Authorization 1 "Bearer token"
}

queries::Q {
    name 1 Hey
    job 0 "Not enabled, won't be included in the request"
}

variables::V {
    some-var 1 "its enabled"
    some-other 0 "its disabled"
}

body::json {
    {
        "from": "Hermes",
        "to": "Hermes"
    }
}

body::form-urlencoded {
    me Hermes
}

body::text {
    "some text, this
    text
can
    be multi-lined,
indentation is respected
    so the line starting with \"some text\" will be indented
and there will also be an empty line."
}

body::multipart-form {
    text name "value"
    file name "path"
}
```

A collection example (this is only some metadata for the collection):

```
metadata {
    type collection
    name "collection name"
}

collection {
    # use None as value if there is no active environment, or just remove the line
    enviroment development
    # collection file can also be in the same folder as the requests in the collection.# collection file can also be in the same folder as the requests in the collection.# collection file can also be in the same folder as the requests in the collection.# collection file can also be in the same folder as the requests in the collection.
    include .
    # allow the usage of collections from other locations
    include /path/to/requests/folder
}

environment::development {
    key value
    key "value with space"
    key "multi
line value"
}

environment::production {
    key value
}
```

To help keep things organized, requests in a collection can be grouped with "folders".
They are not real folders though. This "folders" are just a file that defines which requests should
be grouped together. See below for an example:

```
metadata {
    type folder
    name "folder name"
}

folder {
    include /path/to/requests
    add /path/to/a/request
}
```
