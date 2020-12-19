# dogr

Thin wrapper of [dog](https://dns.lookup.dog/) for easy reverse lookup.

It performs reverse lookup if the single argument is IPv4 or IPv6 address.
Pass arguments to `dog` as is otherwise.

```console
$ dogr 1.1.1.1
PTR 1.1.1.1.in-addr.arpa. 18m53s   "one.one.one.one."

$ dogr 2620:0:2d0:200::7
PTR 7.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.0.0.d.2.0.0.0.0.0.0.2.6.2.ip6.arpa. 5h49m06s   "www.icann.org."

$ dogr example.net @1.1.1.1
A example.net. 23h55m07s   93.184.216.34
```
