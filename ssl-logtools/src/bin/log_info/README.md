# log_info

This tool gives some basic statistics for a given log file. It also serves as an example for using the ssl-logtools-rs library to read a log file.

## Usage

Provide a path to a log file.

```shell
log_info <LOG_PATH>
```

Example output:

```text
Log duration: 1958.093942606s
Refbox2013           128869 msgs  (  65.81 Hz)
Vision2014           116214 msgs  (  59.35 Hz)
VisionTracker2020    297515 msgs  ( 151.94 Hz)
Vision2010           116215 msgs  (  59.35 Hz)
```
