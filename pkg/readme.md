# lfu-fast

LFU (Least Frequently Used) implementation, written in Rust provided as WebAssembly module, using a min-heap, that is, a basic vector, and a hashmap for storage of indices to keep track of cache blocks to allow for constant-time searches.

### Build

```sh
$ make
```

### Usage

```sh
$ yarn add lfu
```

```javascript
const Lfu = require("lfu").Lfu;

const lfu = new Lfu(4);

lfu.refer("1"); // insert 1
lfu.refer("2"); // insert 2
lfu.refer("1"); // incr 1
lfu.refer("3"); // insert 3
lfu.refer("2"); // incr 2
lfu.refer("4"); // insert 4
lfu.refer("5"); // remove 3, insert 5
```
