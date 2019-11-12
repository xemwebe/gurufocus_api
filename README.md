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
//!
The GuruFocus API provides all data in JSON format, and the basic API functions currently 
will just return these JSON structures as `serde_json::Value` types without any further
processing. The `serde_json::Value` types can be deserialized 
into more meaningful data structures, as is demonstrated in the `gurulist` example. 

Please note that the GuruFocus API provides sometimes numerical data as strings rather 
than numbers. Currently, these strings will not be transformed into numbers, though this is 
planned for a future release. Therefore, although the basic API wrapper will not changes, the
custom types like `Guru` are subject to such changes.



