# Hermes Language

A way to define requests and collections for Hermes API client.

A quick example of a typical block definition:

```
block-type.sub-type as my-block {
    field 1 `value`
    another 0 block-2
    one-more 1 block-type.sub-type {
        nested 1 `value`
    }
}
```

A block is a collection of field-value entries defined between curly braces.
Start by using one of the `block-type` below.

Full list of `block-type`:

- collection
- request
- headers
- queries
- environment
- body

A `sub-type` is an extension of a `block-type` that further defines how the block should be read.
As of now, only the `body` and `environment` block has extended type. More on that below.

Blocks can optionally have an identifier linked to them using the `as` keyword after the `block-type` or `sub-type`.
This identifier can be used to reference a block from another block. This can be useful for defining multiple sets of `headers` or `body`.

Example:

```
block-type {
    field 0 `value`
}

block-type as name {}

block-type.sub-type as name {}
```

> Using the same identifier for two different blocks will cause issues.

Fields in a block can have to states, `enabled` or `disabled` which are represented by `1` and `0` respectively.
The state of a field can be used to control whether it should be considered as part of a collection, request or environment.
The state of a field is **optional**, by default, fields without explicit state definition are treated as `enabled` or `1`.

Fields wrapped in bettwen double quotes, allow the field name to start with numbers and have spaces. Though, Hermes will encode and allow
the format, it is generally not a good practice.

## Reserved identifier prefix (self)

The prefix `self` is reserved for some internal identifiers which must not be used for user defined blocks as identifier.
However, the identifiers itself with prefix `self` can be used, just not as an identifier for a block.

- `self-requests`: Refers to all the request blocks in the current `.hermes` file.
- `self-environments`: Refers to all the environment blocks in the current `.hermes` file.

## Folder structure of a hermes collection.

No nested collections.

Opening a collection will only read the root folder `collection.hermes`.

Manage environments with different dotenv files or define `environment` blocks in `collection.hermes`.

```
folder/
    collection.hermes
    some-request.hermes
    some-other-request.hermes
    some-other-folder/
        some-request.hermes
    ...
```

Reserved file names:

- `collection.hermes`: defines basic collection properties.

## Collections

A collection file only defines basic properties such as the collection name, where to find
request files and define environments.
Inside the `collection.hermes`, the first thing that must be defined is the collection block.
From there, the parser will start to go down the `include` keywords and parse the requests.

Request blocks can also be defined in the same file `collection.hermes`. This makes it possible
to manage a small collection without the need of multiple files.

The collection block has a set of key fields that act a certain way:

- `name`: defines the name of the collection.
- `include`: include requests from a path.
- `environment`: This defines which environment the collection should be using. The environment must be defined in the same file `collection.hermes`.

```
collection {
    name `My collection`
    include 1 `.`
    include 1 `./some-other-folder`
    include 1 self_requests
    environment 1 my-env
    environment 1 {
        SOME 1 `value`
    }
    environment 1 `.env`
}

environment as my-env {
    # the value of SOME will be "value", replaced by the next environment.
    SOME 1 `/some/url`
}
```

Environments can be defined as a separate block with an identifier (otherwise it won't be able to be referenced),
inline, or by giving a path to a file that follows the conventional `KEY=VALUE` file format.

The down size of using a file for the environment is that enabling/disabling a key-value pair its not possible
at compile so everything time the key-value pairs will be enabled.

The value of an environment entry will be replaced by the last environment defined in a collection block if they share the same key.

## Requests

A request block contains basic information of one request that belongs to some collection.
Request files are just normal `.hermes` files without the filename constraint like collection files.

```
# some .hermes file in path .
request as my-request {
    url `https://juancwu.dev`
    method `post`
    headers my-headers
    queries my-queries
    body my-body
}

headers as my-headers {
    Content-Type `application/json`
}

queries as my-queries {
    name 1 `my name is ...`
}

body.json as my-body {
    value 1 `
        {
            "name": "Hermes"
        }`
}

```

### Type of body blocks

Body blocks have different `sub-type`s that are supported by Hermes.

- json
- text
- form-urlencoded
- multipart-form

For `sub-type` json and text, the field `value` must be used for Hermes to know where
is the content to use.
Note that for json bodies, one should just write a normal json in between tilts.

```
body.json {
    value 1 `
        {
            "name": "Hermes"
        }`
}

body.text {
    value 1 `some text`
}

body.form-urlencoded {
    field 1 `value`
    field-2 0 `value`
    "1 if this name is needed, but its bad practice" 0 `value`
}

body.multipart-form {
    text-fieldname 1 `some text`
    file-fieldname 1 `/path/to/file`
}
```

## Tokens

Simple set of tokens that can be used to describe all the read characters from an input.

The syntax analyzer would be the one doing the heavy lifting in making sense of the tokens.

- BlockType(String)
- SubBlockType(String)
- Identifier(String): this can be an identifier for a block or a field.
- Digit(u8): it is an usigned 8bit integer since the only digits that should appear are 1 or 0.
- StringValue(String)
- Delimeter(char)
- Unknown(char)
