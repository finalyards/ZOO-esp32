# Who invited `alloc`?

Making `no_std` Rust applications, sometimes a dependency comes along - which might be transitive - that wants to have an allocator. You know this by seeing:

```
$ cargo build [...]
[...]
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait
```

The problem is, Cargo doesn't state *who* needs the allocator.

## Solution 1 - just give in!

Add to `Cargo.toml`:

```
esp-alloc       = { version = "0.5.0" }
```

..and to your code:

```
fn init_heap() {
    use core::mem::MaybeUninit;

    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}
```

..which you of course need to call. Problem - solved?

## Solution 2 - find the culprit



## Solution 3 - fake it

Do this:

```rust
use alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        0 as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unreachable!();     // since we never allocate
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
```

Problem solved. The party that wants to *link* to a global allocator doesn't necessarily ever *call* it. If it were to call, returning a C-`malloc`-null-pointer from `alloc` is perfectly valid, and indicates "out of memory".

This worked for our `tof/vl53l5cx` project.  


## References

- ["no global memory allocator found but one is required; [...]"](https://stackoverflow.com/questions/74012369/no-global-memory-allocator-found-but-one-is-required-link-to-std-or-add-glob) (SO, Oct'22)
- ["Rust no_std find why global memory allocator is required"](https://users.rust-lang.org/t/rust-no-std-find-why-global-memory-allocator-is-required/77679/2) (Rust-lang discussion, Jun'22)
