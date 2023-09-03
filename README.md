# cidr2ip
A Rust program that reads a file containing IPs and CIDR, expands the CIDR ranges to individual IPs, and prints them to standard output.

## Examples

### Input
```
192.168.1.0/28
172.16.0.1
10.0.0.23/30
```

### Output
```
192.168.1.0
192.168.1.1
192.168.1.2
192.168.1.3
192.168.1.4
192.168.1.5
192.168.1.6
192.168.1.7
192.168.1.8
192.168.1.9
192.168.1.10
192.168.1.11
192.168.1.12
192.168.1.13
192.168.1.14
192.168.1.15
172.16.0.1
10.0.0.23
10.0.0.24
10.0.0.25
10.0.0.26
```
