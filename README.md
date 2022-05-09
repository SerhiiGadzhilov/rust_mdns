# rust_mdns
Rust mdns demo application

## Compilation
1. Checkout the source code
2. Compile using cargo (cargo build)

## Usage
There are the following main options:
```
-d, --discovery    Discovery mdns service (Default option)
-s, --service      Start mdns service
```

On the server machine run rust_mdns with the option --service.
```
Start service
Local IPv6 address: fe80::b540:c897:268:480e%14
Local IPv4 address: 192.168.56.1
Opened 2 sockets for mDNS service
Service mDNS: _test-mdns._tcp.local.:42424
Hostname: DESKTOP
Query PTR _services._dns-sd._udp.local.
  --> answer _test-mdns._tcp.local. (unicast)
```

On client machine run rust_mdns with the option --discovery
```
Local IPv6 address: fe80::b540:c897:268:480e%14 <br>
Local IPv4 address: 192.168.56.1 
Local IPv6 address: fe80::979:5625:c1ca:b489%3
Local IPv4 address: 192.168.0.106
Opened 4 sockets for DNS-SD
Sending DNS-SD discovery
Reading DNS-SD replies
192.168.0.101:5353 : answer _services._dns-sd._udp.local. PTR _test-mdns._tcp.local. rclass 0x8001 ttl 10 length 18
```
