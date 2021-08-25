# Asciinet, Networking as Ascii Art!

## What is it?

Asciinet is a command line tool that shows the host networking topology as
ascii art.

### Concept

Currently, the tool is not finished (not even started), but the basic idea is:

1. The user will run the asciinet command, they will be able to include a list
   interfaces in order to filter them.

2. The user will get an output similar to this:
```
+---------+--------+--+--------+-------+-------+--+-------+----------+
|         |  ETH1  |  |  ETH2  |       | ETH3  |  | ETH4  |          |
|         +--------+--+--------+       +-------+--+-------+          |
|         |       BOND1        |       |      BOND2       |          |
|         +---------+----------+       +--------+---------+          |
|                   |                           |                    |
|                   |                           |                    |
|                   |                           |                    |
|                   |                           |                    |
|                   |                           |                    |
|                   |                           |                    |
|                   |                           |                    |
|            +------v-----+             +-------v------+             |
|            |  VLAN-PROD |             |  VLAN-MGMT   |             |
|            +------------+             +--------------+             |
|                                                                    |
|                                                                    |
|                                                                    |
|  HOST                                                              |
+--------------------------------------------------------------------+
```

### What is it using?

* Rust programming language!

* It is using [Nispor](https://github.com/nispor/nispor) in order to gather the
  network information from kernel. This information is updated as it is using
  netlink to get gather it.

* We have two options:

  1. Create an asciiart library for rust, this library will be use to draw the
     network state as ascii in the stdout.

  2. Use a image generator library and then convert it to asciiart with another
     library.

## TODO list
* ALL :D
