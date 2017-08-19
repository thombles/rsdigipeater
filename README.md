# rsdigipeater
Userspace packet radio digipeater for Linux. When running, this enables digipeating between active AX.25 interfaces (ax0, ax1, etc.). 

This is a rewrite of the venerable axdigi with a few improvements. This uses the more modern AF_PACKET raw sockets interface on Linux which doesn't have problems with reporting duplicate packets. It also detects and uses AX.25 interfaces that have no IP address configured.

# Licence

rsdigipeater
Copyright (C) 2017 Thomas Karpiniec

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
