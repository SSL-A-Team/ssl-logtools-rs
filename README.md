# ssl-logtools-rs

Rust library and tools for working with RoboCup Small Size League [game logs](https://ssl.robocup.org/game-logs/).

This repo is NOT intended to replace existing league tools for working with game logs (such as [ssl-logtools](https://github.com/RoboCup-SSL/ssl-logtools) and [ssl-go-tools](https://github.com/RoboCup-SSL/ssl-go-tools)). This package aims to extend the existing ecosystem with Rust-native utilties.

## Tools

- [log_info](src/bin/log_info/README.md)
- [video_overlay_gen](src/bin/video_overlay_gen/README.md)

## Library Features

### Reading log files

The primary mechanism for reading log files with this library is the `LogFileReader` struct. For convenience, this struct implements the `Iterator` trait so you can directly loop over messages from your log file.

```rust
use ssl_logtools_rs::MessageBody;
use ssl_logtools_rs::log_file_reader::LogFileReader;

let reader = LogFileReader::new("path/to/log/file.log")?;
for message in reader {
    match message.body {
        MessageBody::Refbox2013(ref_data) => { /*...*/ },
        // ...
    }
}
```

If you are loading log data from another data source (not a file on disk), you can pass any implementation of `Read` to `extract_next_message`.

There is also a convenienve function for pulling all referee messages out of a log file.

```rust
use ssl_logtools_rs::get_all_referee_messages;

let ref_messages = get_all_referee_messages("path/to/log/file.log")?;
// ref_messages is a Vec<LogMessage>
```
