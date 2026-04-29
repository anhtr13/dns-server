# The ["Build Your Own DNS server" Challenge](https://app.codecrafters.io/courses/dns-server/overview)

[![progress-banner](https://backend.codecrafters.io/progress/dns-server/c2c1b7eb-b7db-4d28-8b31-8fac642faff4)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

A DNS server that's capable of parsing and creating DNS packets, responding to DNS queries, handling various record types and doing recursive resolve.
Build to learn about the DNS protocol, DNS packet format, root servers, authoritative servers, forwarding servers, various record types (A, AAAA, CNAME, etc) and more.

## Build & Run

```bash
# Release build
cargo build --release

# Use --resolve flag to forwarding DNS queries to a specified DNS server
./target/release/dns-server --resolver [address]
```

Or use the project script (builds then runs):

```bash
./your_program.sh --resolver [address]
```
