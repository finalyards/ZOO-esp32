# Reason to avoid `esp_alloc`

Strange things happen if you bring in `esp_alloc` in the `Cargo.toml`. This write is to remind of that.

## Root cause?

Something is bringing in the need for allocation, though we don't use it, ourselves.

The best way to deal with this seems to be to fake an allocator (that would then catch anyone using it):

```
use core::alloc::{GlobalAlloc, Layout};

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

This is the **route of least magic**. The code's there.

The root cause (who brings `alloc` in) remains unknown... `#18-Sep-25`

### But isn't `esp_alloc` for this?

Sure. 

If we bring it in, 

What did work for `single-emb` 




- `single-emb` worked fine