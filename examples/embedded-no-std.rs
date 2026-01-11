//! Typical error handling in no-std, but alloc, embedded environments.
#![allow(
	clippy::undocumented_unsafe_blocks,
	clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	reason = "Example"
)]

extern crate alloc;

use ::alloc::alloc::{GlobalAlloc, Layout};
use ::neuer_error::{CtxError, Result, traits::*};
use ::core::{
	cell::UnsafeCell,
	ptr::null_mut,
	sync::atomic::{AtomicUsize, Ordering},
};


fn self_test() -> Result<()> {
	Err(CtxError::new("Memory error"))
}

fn boot_up() -> Result<()> {
	self_test().context("Defect hardware detected")?;
	Ok(())
}

fn run() -> Result<()> {
	boot_up().context("Booting failed")?;
	Ok(())
}

fn main() {
	run().context("Failed running software").unwrap();
}


/* Stolen from Rust docs, custom allocator. You will use something else, probably. */
const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct SimpleAllocator {
	arena: UnsafeCell<[u8; ARENA_SIZE]>,
	remaining: AtomicUsize, // we allocate from the top, counting down
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
	arena: UnsafeCell::new([0x55; ARENA_SIZE]),
	remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let size = layout.size();
		let align = layout.align();

		// `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
		// So we can safely use a mask to ensure alignment without worrying about UB.
		let align_mask_to_round_down = !(align - 1);

		if align > MAX_SUPPORTED_ALIGN {
			return null_mut();
		}

		let mut allocated = 0;
		if self
			.remaining
			.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |mut remaining| {
				if size > remaining {
					return None;
				}
				remaining -= size;
				remaining &= align_mask_to_round_down;
				allocated = remaining;
				Some(remaining)
			})
			.is_err()
		{
			return null_mut();
		};
		unsafe { self.arena.get().cast::<u8>().add(allocated) }
	}
	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
