# Hermes Language

A way to define requests and collections for Hermes API client.

## Language Block Summary

-   Block Type
-   SubBlock Type
-   Identifier
-   Digit
-   Value
    -   Single Line Value
    -   Multi-line value (raw)

## Reading Block Type, SubBlock Type, and Identifier

Blocks just identifiers. Reading them can be treated as such too.

```
S - (A..Z,-)
    -> Read Block/Identifier - (*) -> End (Block/Identifier)
```

Then, sub block types need to be treated differently for the sake of making
parsing and syntax analyzing easier.

```
S - (.)
    -> Expect Read Sub Block Type - (A..Z,-) -> Read Sub Block Type - (*) -> End (Sub Block Type)
    - (*) -> End (Error)
```

## Reading Digit (Single digit only)

```
S - (0..9) -> End (Digit)
```

## Reading Value (Single or Multi line)

Single line value consist of starting double quotes and must end with double quotes.

```
S - (") -> Expected character or double quote
    -> Single Line Value - (anything but double quotes) -> Loop back
    -> Single Line Value - (") -> End (Single Line Value)
    -> Single Line Value - (backslash) -> Escape Character - (*) -> Single Line Value
```

```
S - (`) -> Expected anything or tilt
    -> Multi Line Value - (anything but tilt) -> Loop back
    -> Multi Line Value - (tilt) -> End (Multi Line Value)
    -> Multi Line Value - (backslash) -> Escape Character - (*) -> Multi Line Value
```

## Proper Language Definition

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

-   collection
-   request
-   headers
-   queries
-   environment
-   body

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
