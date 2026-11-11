# Transposition-table memory in Remus5 and Remus6

Remus5, Remus6, Remus5-Rust, and Remus6-Rust each budget half the computer's
installed physical RAM for their native transposition table. They read that value
once at startup through the Windows `GetPhysicallyInstalledSystemMemory` API.
The calculation does not use free RAM, available RAM, or RAM currently in use.
These engines target 64-bit Windows 11.

A transposition table stores search results for positions reached through different
move orders. A larger table can retain more results. It does not change the chess
rules, evaluation terms, or search algorithms. Remus6's Lichess response cache is
a separate, small cache and does not use this budget.

## Capacity

| Installed RAM | Target table budget | Python entry threshold | Rust buckets |
| ---: | ---: | ---: | ---: |
| 8 GiB | 4 GiB | 16,519,104 | 67,108,864 |
| 16 GiB | 8 GiB | 33,038,209 | 134,217,728 |
| 24 GiB | 12 GiB | 49,557,314 | 201,326,592 |
| 32 GiB | 16 GiB | 66,076,419 | 268,435,456 |
| 64 GiB | 32 GiB | 132,152,839 | 536,870,912 |

One GiB is 1,073,741,824 bytes. Each Rust bucket holds four 16-byte entries. The
budget covers the table, not the engine's other memory or the chess GUI.

## Python: an approximate budget

`transposition_table.py` uses:

```python
MAX_ENTRIES = installed_bytes // 2 // 260
```

The implementation keeps at least one entry for a positive RAM report. The
260-byte value estimates the memory cost of an entry; it does not enforce a
byte-level ceiling. Dictionary capacity, Python objects, allocator overhead, and
resizing peaks affect actual memory use. The dictionary allocates as entries are
stored, so startup does not allocate the full target.

The table retains its clear-on-limit policy. The check occurs before an insertion
and uses `len(table) > MAX_ENTRIES`, allowing one entry above the threshold until
the next store. A failed cache insertion raises `MemoryError`; the handler clears
the dictionary and skips that insertion so native search can continue. This
handler cannot guarantee recovery from memory exhaustion elsewhere in the engine.

## Rust: whole buckets and multiply-shift indexing

`transposition_table.rs` computes the number of whole 64-byte buckets that fit in
half the installed RAM. It bounds the allocation to Rust's pointer-arithmetic
limits. On supported 64-bit machines, normal installed-memory sizes fit directly.

Both `store()` and `probe()` use the same index:

```rust
((u128::from(key) * bucket_count as u128) >> 64) as usize
```

This maps the full unsigned 64-bit key range into indexes from zero through
`bucket_count - 1`. Any positive bucket count works. For example, 24 GiB of RAM
supports a 12 GiB table rather than rounding down to 8 GiB.

The four-entry bucket layout, score bounds, mate-distance normalization, and
age/depth replacement policy remain the same. Key zero marks an empty entry and
is not cached. A full bucket can require replacement even when most of the whole
table is empty; changing the index can therefore change collisions and search
node counts. Python and Rust have different replacement policies.

Windows `VirtualAlloc` reserves and commits the table region, with zero-filled
pages supplied on first access. The engine does not explicitly zero the entire
budget at startup. Windows still charges the allocation against system commit
capacity, which includes RAM and paging-file backing. `VirtualFree` releases the
region when the table is dropped. No additional Rust crate is needed for these
Windows calls.

## Failure handling and simultaneous engines

If Windows cannot report installed RAM, Python uses a two-million-entry limit and
Rust requests 512 MiB. Rust handles a failed allocation by retrying with half as
many buckets until allocation succeeds. If every attempt fails, it skips table
storage and returns misses from probes, allowing native search without caching.
The normal target is never recalculated from current memory usage; a smaller
allocation is an explicit failure path.

The budget is per engine process. Two running engines can each target half the
installed RAM, leaving no table-budget allowance for Windows, the GUI, or other
applications. Actual use may grow during play and can cause paging. Use one
experimental engine process at a time when the machine needs that remaining
memory for other work. The capacity is not a shared pool across engines.

Remus3 and Remus4 retain their two-million-entry Python limits. Remus4-Rust retains
its fixed 512 MiB table. The five-day workshop uses those versions.

## References

- [Installed physical RAM API](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getphysicallyinstalledsystemmemory)
- [VirtualAlloc and commit behavior](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc)
- [VirtualFree and region release](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree)
