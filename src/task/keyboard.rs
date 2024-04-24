use conquer_once::spin::OnceCell;
use core::{
    pin::Pin, 
    task::{Context, Poll}
};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crossbeam_queue::ArrayQueue;
use crate::print;
use crate::println;

// The idea is that the poll_next implementation stores the current waker in this static, 
// and the add_scancode function calls the wake function on it when a new scancode is added 
//to the queue.
static WAKER: AtomicWaker = AtomicWaker::new();

/*
    could use the lazy statuc
    Instead of the OnceCell primitive, we could also use the lazy_static macro here. 
    However, the OnceCell type has the advantage that we can ensure that the 
    initialization does not happen in the interrupt handler, 
    thus preventing the interrupt handler from performing a heap allocation.

*/

pub struct ScancodeStream {
    // don't allow prevent construction of the strct from outside of the module
    // only the new can construct the stream
    // in short, prevents having a public constructor
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        
        // fast path
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());

        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}
pub async fn print_keypress() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);

        while let Some(scancode) = scancodes.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => print!("{}", character),
                        DecodedKey::RawKey(key) => print!("{:?}", key),
                    }
                }
            }
        }
}

// ----------------------------------------------------------------
// Interupt Context
// ----------------------------------------------------------------


static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
/// Since this function should not be callable from our main.rs, 
/// we use the pub(crate) visibility to make it only available to our lib.rs.
/// 
/* 
    IMPORTANT
    It is important that we call wake only after pushing to the queue because 
    otherwise the task might be woken too early while the queue is still empty. 
    This can, for example, happen when using a multi-threaded executor that starts 
    the woken task concurrently on a different CPU core. While we don’t have thread 
    support yet, we will add it soon and don’t want things to break then.
*/
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}