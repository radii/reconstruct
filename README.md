reconstruct - use a list of block hashes to allow re-construction of a target file from a variety of
inputs.

Suppose you have a large file (an OS disk image perhaps) in multiple versions, with updates that
need to be distributed across low-bandwidth network links.  This `reconstruct` program allows
flexible and efficient implementation of update distribution, with flexible utilization of data
sources on the destination side.

The operation of `reconstruct` consists of two parties, `source` and `destination`.  (Of course
multiple destinations are possible, and much of the work done by `source` is amortized across
destinations.)  A reconstruction proceeds as follows:

 1. `source` hashes `v2.img` to create `v2.img.rcn` reconstruction list.
 2. optionally, `source` compares against a `v1.img` and generates a `v2.img.drcn` (delta) file.
 3. the `.rcn` and optional `.drcn` are compressed and transmitted to `destination`.
 4. `destination` reads the `v2.img.rcn`, and reconstructs `v2.img.tmp` based on any inputs that
    are available. This might include a `v1.img`, a modified version of `v1.img` (such as a live
    filesystem image that has been modified), other similar filesystem images, and any `.drcn`
    sent along from `source`.
 5. `destination` either succeeds in fully reconstructing `v2.img`, or generates a list of
    blocks that are unavailable and writes the list to `v2.img.nrcn` (needed).
 6. The `.nrcn` needed list is returned to `source` which constructs the necessary `.drcn` to
    complete the reconstruction.
 7. The final `.drcn` is sent to `destination` and the final reconstruction is completed.

The core idea is to treat the `target` as a list of 4 KiB blocks, construct an efficient
representation of `target`, and use diverse inputs on the destination to reduce network traffic to a
minimum. Think of `reconstruct` as kind of like asynchronous rsync.


Hash tree representation
------------------------

`reconstruct` uses SHA256 with a hash tree structure.  The `target` blocks are 4KiB. Each data block
is named `b{index}` where index denotes position from beginning of the file. Leaf hashes are
truncated to 32 bits and fan-out factor of 256 is used.  The hash tree nodes are named
`h{index}_{level}` where `index` denotes position and `level` denotes level in the hash tree; the
level-0 hashes (truncated to 32 bits) can be named `h{index}_0` for completeness or `h{index}` for
short.

The level 1 hashes are computed across the data extent spanned. Levels 2 through M are computed
across the next-lower level hashes, rather than the data.  Here is some ASCII art showing the tree
for levels 0, 1, and 2:

```
                                        h0_2
                                          |
               +--------------------------+--------------------+
               |                                               |
               h0_1             h1_1        ......        h255_1
                 |                |
+----------------+------+ +-------+------+
|                       | |              |
|h0  h1  h2  h3  ... h255 h256 .....  h511 h512 .... hN
|b0  b1  b2  b3  ... b255 b256 .....  b511 b512 .... bN
```

And by example,
```
h0 = truncate_32_bits(SHA256(b0))
h1 = truncate_32_bits(SHA256(b1))
h0_1 = SHA256(b0 | b1 | b2 ... | b255)
h1_1 = SHA256(b256 | b257 ... | b511)
h0_2 = SHA256(h0_1 | h1_1 | h2_1 ... h255_1)
```

Reconstruction process
----------------------

Hash collisions
---------------

Since the leaf hashes are only 32 bits, hash collisions are inevitable and expected.  In a 4GB image
a million blocks are present and around 128 collisions are expected. Each input block matching the
truncated hash is treated as a "possible input" and all possible inputs are considered when
constructing the final level 1 block.  Since the level 1 hash is across the input blocks rather than
across the level 0 hashes, this weakness is not present at higher levels.

Malicious inputs
----------------

The collision resolution becomes expensive if there are many leaf hash collisions within a single
level 1 hash; in the worst case of every block having two possible input blocks, 2^256 possible
arrangements would need to be considered.  This can be detected during the distillation process and
the process terminated.  A possible mitigation is to swap out the hash function from SHA256 to
HMAC_SHA256 with the top-level SHA256 as the salt for the interior and leaf hashes.

File formats
------------

The 
