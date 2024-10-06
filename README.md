# udp-data-pipeline

## Notes


Links I used
- [Understanding Ip Multicasting](http://www.steves-internet-guide.com/introduction-multicasting/)
- [Udp Multicast setup](https://lcm-proj.github.io/lcm/content/multicast-setup.html)
- [configuring multi cast](https://www.reddit.com/r/networking/comments/ouarlx/configuring_multicast/)

---

To observe local multicast (I used to confirm publisher was sending messages)
```bash
tcpdump multicast
```
To send a message to an address (I used to confirm subscriber)
```bash
echo "Hello, from bash" | nc -u -w1 <IP-ADDRESS> <PORT>
```
example that worked for me
```bash
echo "Hello, from bash" | nc -u -w1 239.255.255.250 1900
```

---

The address **239.255.255.250** using port **1900** is definitely not a good one to use but it was the first one I was able to cast through sucessfully. The biggest issue is I am receiving messages that are not form my publisher. 

## Get started



```rust

```

For more advanced examples, please have a look at the following section.

