[![Workflow Status](https://github.com/enarx/telestes/workflows/test/badge.svg)](https://github.com/enarx/telestes/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/telestes.svg)](https://isitmaintained.com/project/enarx/telestes "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/telestes.svg)](https://isitmaintained.com/project/enarx/telestes "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# telestes

## Welcome!

Telestes provides an implementation of Media Types; also formerly known
as MIME Types. This crate emphasizes performance and correctness; as well
as the ability to parse in a `const` context. It is `no_std`-enabled.

To get started, check out the [`MediaType`] and [`Essence`] types.

## Optional Features

* `alloc` - enables the use of types in the `alloc` crate
* `serde` - enables serialization/deserialization using `serde`
* `db`    - enables the `const` database of IANA Media Types

## Terminology

The terminology of this crate should follow the terminology broadly used
in the RFCs or other notable documentation; with one exception. Since the
`type` terminology of a Media Type collides with the Rust `type` keyword,
we have chosen the terms `genus` and `species` to replace the terms for
`type` and `subtype`, respectively.

## Example

```rust
use telestes::MediaType;

let mt = MediaType::new("text/plain; charset=UTF-8").unwrap();
assert_eq!(mt, "text/plain; charset=UTF-8");
assert_eq!(mt.essence(), "text/plain");
assert_eq!(mt.essence().genus(), "text");
assert_eq!(mt.essence().species(), "plain");
assert_eq!(mt.parameters().get("charset"), Some("UTF-8"));
```

## Naming

This crate is named after the character Telestes from the Greek play Seven
Against Thebes by Aeschylus. This character is possibly history's first
recorded pantomime.

License: Apache-2.0
