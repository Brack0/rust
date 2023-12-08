# Doom Rand performance

## Trivia

This is a small application to compare the fake random from Doom©️ with common random functions.
I assume that the RNDTABLE will stay in L1 cache as it is used really often in the game (multiple times per game tick).

## Results

### Hardware

```
system         PowerEdge T320
processor      Intel(R) Xeon(R) CPU E5-2420 v2 @ 2.20GHz
memory         192KiB L1 cache
memory         1536KiB L2 cache
memory         15MiB L3 cache
memory         16GiB System Memory
memory         8GiB DIMM DDR3 Synchronous Registered (Buffered) 1600 MHz (0,6 ns)
memory         8GiB DIMM DDR3 Synchronous Registered (Buffered) 1600 MHz (0,6 ns)
```

### Output

```
Time for 10,000,000 calls of Doom rand : 1.391745ms
Time for 10,000,000 calls of OS rand : 7.656156546s
Time for 10,000,000 calls of Thread rand : 99.830306ms
```
