# Problems To Be Solved

# Test tasks

- Build a environment in Ubuntu in home.


# To be finished

- What is gpfdist request looks like? Where is file path?
  - Find a way to print the request in string.
  - Debug the gpdb in the server end.

- What is gpfdist header? What value should be added into response header?

# Blog Topic
## Error handling
Some(std::env::current_dir().expect("Failed to get current directory"))
behavior of function `unwrap()`
How to map error
The behavior of `?`

## tokio
## HTTP Struct
## String Vs &str

# Done

- What is gpfdist request looks like? Where is file path?
File path is in `Request Line`.
```bash
# Method   Request Target   HTTP Version
GET /data.txt HTTP/1.1  # Request Line
# Key   Value
Host: 127.0.0.1:8080  # Request Headers
Accept: */*
X-GP-XID: 137-0000000005
X-GP-CID: 5
X-GP-SN: 1
X-GP-SEGMENT-ID: 0
X-GP-SEGMENT-COUNT: 1
X-GP-LINE-DELIM-LENGTH: -1
X-GP-ZSTD: 1
X-GP-MASTER_HOST: 10.148.194.73
X-GP-MASTER_PORT: 15432
X-GP-PROTO: 1
X-GP-COORDINATOR_HOST: 10.148.194.73
X-GP-COORDINATOR_PORT: 15432
X-GP-CSVOPT: m0x 92q  0n0h0
X-GP_SEG_PG_CONF: /home/gpadmin/gpdb6_data/primary/gpseg0/postgresql.conf
X-GP_SEG_DATADIR: /home/gpadmin/gpdb6_data/primary/gpseg0
X-GP_SEG_LOGDIR: log
X-GP-DATABASE: gpadmin
X-GP-USER: gpadmin
X-GP-SEG-PORT: 7000
X-GP-SESSION-ID: 137
# Empty Line
# Request Body
```

CREATE EXTERNAL TABLE my_external_table (
    C1 TEXT
)
LOCATION (
    'gpfdist://localhost:8080/data.txt'
)
FORMAT 'TEXT'
LOG ERRORS SEGMENT REJECT LIMIT 1000 ROWS;