use super::Task;
use alloc::collections::VecDeque;
use core::task::{Waker, RawWaker, RawWakerVTable, Context, Poll};

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

// can be Self
impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task) // push pop on both ends
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {} // task done
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

// manually creating a RawWaker is dangerous because the task queue is not initialized
// but ther is no other way to create a RawWaker that does nothing
fn dummy_raw_waker() -> RawWaker {
    // create an associated RawWaker V Table
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    // clone operations with ...clone
    //RawWakerVTable::new
    //Creates a new RawWakerVTable from the provided clone, wake, wake_by_ref, and drop functions.
    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
    // The passed *const () does not matter since none of the vtable functions use it. 
    // For this reason, we simply pass a null pointer.
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}


/*

    The RawWaker type requires the programmer to explicitly 
    define a virtual method table (vtable) that specifies 
    the functions that should be called when the RawWaker is 
    cloned, woken, or dropped. 

    *const () pointer, which is the self reference
    Raw Waker Type should be non-generic but still support ...but still support
    non-generic types.

*/