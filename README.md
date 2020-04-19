# GuruFocus API

This project provides a set of functions to receive data from the 
the guru focus website via the [GuruFocus API](https://www.gurufocus.com/api.php). This project 
is licensed under Apache 2.0 or MIT license (see files LICENSE-Apache2.0 and LICENSE-MIT).

# Usage
Please note that you need at least a premium account to use this API. There a couple of 
examples demonstrating how to use the API in your own rust projects. To run this example,
you first need to define an environment variable holding the user Token you got from
GuruFocus:
```bash
export GURUFOCUS_TOKEN='<your user token>'
```

The examples can be executed via the command
```dummy
cargo test --example <name of example>
```
Here, `<name of example>` could be the name of any of the files in the examples folder
without the `.rs` extension
Please note that running any of the examples increases your API access counter by at least 1.

The GuruFocus API provides all data in JSON format, and the basic API functions currently 
will just return these JSON structures as `serde_json::Value` types without any further
processing. The `serde_json::Value` types can be deserialized 
into more meaningful data structures, as is demonstrated in the `gurulist` example. 


The GuruFocus API returns numbers sometimes as numbers, sometimes as strings.
This is dealt with by introducing a new struct `FloatOrString` containing a
float value, but which can be read from either a string or float automatically.
The drawback is that `.0` as to be added to the variable name of a specific data
structure. I.e., to access the quoted price in a variable of type Quote, i.e.
`q: Quote`, the price can be accessed via `q.price.0` (if accessible at all)
instead of `q.price`, or, more generally, converted to `f64` by
`q.price.into()`. In a few cases, the string contains not a number, but an error
message, like "Negative Tangible Equity". In such cases, if the string can not
be parsed to a number, the value is set to `NAN`.
 
Please note that the library is not yet stable and that the user interface is
still subject to change. However, feedback regarding the usability and
suggestions for improving the interface are welcome.

