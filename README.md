Library to represent an IGMPv2 Multicast Group State machine as defined in RFC 2236

 ----------------------------|                |<-----------------------
|                            |                |timer expired           |
|               timer expired|                |(notify routing -,      |
|          (notify routing -)|   No Members   |clear rxmt tmr)         |
|                    ------->|    Present     |<-------                |
|                   |        |                |       |                |
|v1 report rec'd    |        |                |       |  ------------  |
|(notify routing +, |        |________________|       | | rexmt timer| |
| start timer,      |                    |            | |  expired   | |
| start v1 host     |  v2 report received|            | | (send g-s  | |
|  timer)           |  (notify routing +,|            | |  query,    | |
|                   |        start timer)|            | |  st rxmt   | |
|         __________|______              |       _____|_|______  tmr)| |
|        |                 |<------------       |              |     | |
|        |                 |                    |              |<----- |
|        |                 | v2 report received |              |       |
|        |                 | (start timer)      |              |       |
|        | Members Present |<-------------------|    Checking  |       |
|  ----->|                 | leave received     |   Membership |       |
| |      |                 | (start timer*,     |              |       |
| |      |                 |  start rexmt timer,|              |       |
| |      |                 |  send g-s query)   |              |       |
| |  --->|                 |------------------->|              |       |
| | |    |_________________|                    |______________|       |
| | |v2 report rec'd |  |                          |                   |
| | |(start timer)   |  |v1 report rec'd           |v1 report rec'd    |
| |  ----------------   |(start timer,             |(start timer,      |
| |v1 host              | start v1 host timer)     | start v1 host     |
| |tmr    ______________V__                        | timer)            |
| |exp'd |                 |<----------------------                    |
|  ------|                 |                                           |
|        |    Version 1    |timer expired                              |
|        | Members Present |(notify routing -)                         |
 ------->|                 |-------------------------------------------
         |                 |<--------------------
 ------->|_________________| v1 report rec'd     |
| v2 report rec'd |   |   (start timer,          |
| (start timer)   |   |    start v1 host timer)  |
 -----------------     --------------------------
