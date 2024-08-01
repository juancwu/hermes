# Grammar Definition

This describes the grammar definition for `.hermes` files.

Very simple language that defines block-wise information.

```
block_type::block_name {
    # values are whitespace separated, except for values surrounded with double quotes
    field_or_key value [...values]
}
```

A `block_type` is like the `var` keyword and the `block_name` is the identifier.

You can reference another block inside a block by using the `block_type` as field
and the `block_name` as the value.

Values can have replaceable placeholders using `{{env_key}}`. They `env_key` will be taken
from the active environment in defined in a `collection` block.

Not all blocks need a `block_name`. To define a block without a name just use `block_type` without
`::block_name`. Hermes will load the block but it won't be available for reference in other blocks.

Example of a block without name:

```
block_type {
    field value
}
```

Special block type `body`. This block can have different content within the block.

Examples:

```
body::json {
    # must be a valid JSON
    {
        "name": "Hermes"
    }
}

body::form-urlencoded {
    # normal field value
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
    # follow pattern: type name value
    text name "value"
    file name "path"
}
```

The correctness of a file won't be checked until reaching the parsing stage of the interpreter.
If there are any errors during any stage of the interpretation of `.hermes` files, it should
abort and report the error as accurate as possible. After that, it should move onto the next
`.hermes` file.
