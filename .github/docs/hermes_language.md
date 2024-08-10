# Grammar Definition

Alright, lets re-define this whole this, again.

This describes the grammar definition for `.hermes` files.

Very simple language that defines block-wise information.

```
block-type.sub-type::block-identifier {
    # values without quotes are block identifiers, this can be used to reference another block
    # just like use the content of that block in this field
    field 0 block-identifier

    # value wrap in double quotes can include whitespaces and escaping characters.
    # however, new lines can't be included since it will result in 'n' for '\n'.
    field-two 0 "value"

    # raw values are versatile, it is possible to include any character until to the.
    field-three 1 r#"multiline, raw
        string that that includes quotes(")"#

    # the number after the field means whether the field is enabled or not.
    # can be used to control what to headers to include for example.
}
```

A block is a collection of field-value entries defined between curly braces.
A `block-type` is built-in keywords of the type of block being defined.

Full list of `block-type`:

- collection
- request
- headers
- queries
- environment
- body

A `sub-type` is an extension of a `block-type` that further defines how the block should be read.
As of now, only the `body` and `environment` block has extended type. More on that in below.

Blocks can optionally have an identifier linked to them. This identifier can be used to reference
a block from another block. This can be useful for defining multiple sets of `headers` or `body`.

Example of a block without name:

```
block-type {
    field 0 value
}
```

Folder structure of a hermes collection.

```
folder/
    collection.hermes
    some-request.hermes
    some-other-request.hermes
    some-other-folder/
        some-request.hermes
    ...
```

No nested collections.

Opening a collection will only read the root folder `collection.hermes`.

Manage environments with different dotenv files.

Now with the folder structure defined, we can define what the `collection.hermes` should have.

```
# must be the first block in the file
collection {
    name 1 "My collection"
    include 1 "." # include all requests from current directory
    include 1 "./some-other-folder"
    include 1 my-request
    environment 1 my-env
}

environment::my-env {
    URL "/some/url"
}

environment.file::file-env {
    path ".env"
}

request::my-request {
    url "https://juancwu.dev"
    method "post"
    headers my-headers
    queries my-queries
    body my-body
}

headers::my-headers {
    Content-Type "application/json"
}

# empty query
queries::my-queries {}

body.json::my-body {
    value 1 r#"
    {
        "name": "Hermes"
    }
"#
}
```

That is an extensive example. Note that requests don't have to be defined in the same file.

Different types of body blocks.

```
body.json {
    value 1 r#"
    {
        "name": "Hermes"
    }
"#
}

body.text {
    value 1 "some text"
}

body.form-urlencoded {
    field 1 value
    field-two 0 value
}

body.multipart-form {
    text-fieldname 1 "some text"
    file-fieldname 1 "/path/to/file"

    # notice the prefix text- or file-
    # that is required for hermes to differentiate between
    # the different type of entry. Substitude the fieldname
    # to the desired name that is used in the form.
}
```
