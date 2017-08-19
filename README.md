# rsdigipeater
Userspace packet radio digipeater for Linux. When running, this enables digipeating between active AX.25 interfaces (ax0, ax1, etc.). 

This is a rewrite of the venerable axdigi with a few improvements. This uses the more modern AF_PACKET raw sockets interface on Linux which doesn't have problems with reporting duplicate packets. It also detects and uses AX.25 interfaces that have no IP address configured.
