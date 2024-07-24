# Hermes

Hermes is a light-weight API client in the terminal with VIM keymaps.

## How it works

Hermes API routes will be grouped into collections. Each collection can have individual routes or
collections. Yes, collections within a collection.

Each collection can have different sets of environments they can run on.
An environment is just like any `.env` file. Nested collections will work by scopes.
The closest scope will take priority and bubbles up.

A route include the method, url, headers, body, etc... Basically anything that a normal API route would have.
You can define a route within Hermes or create a file within the collection folder with the extension `.hermes`. The file
its just a normal text file.

Here is an example of a route:
```
# greet.hermes -- this is a comment
metadata {
    name: Greet Hermes
}

route {
    method: post
    url: https://{{HOST}}:8000/greet
    # select the body to use
    body: json
    headers: {
        Content-Type: application/json
        Authorization: Bearer token
    }
}

body::json {
    {
        "from": "Hermes",
        "to": "Hermes"
    }
}

# define multiple bodies, and use them later if you need them
body::form-urlencoded {
    me: Hermes
}
```

An environment can also be define in Hermes or create a file within the collection folder with name `.env`.
The contents of the file is just a typicla `.env` file. For nested environments, just create a new `.env` in the nested collection.
