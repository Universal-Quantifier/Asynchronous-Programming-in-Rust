mod future;
mod http;
mod runtime;
use future::{Future, PollState};
use runtime::Waker;

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

// =================================
// We rewrite this:
// =================================

// coroutine fn async_main() {
//     let mut counter = 0;
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").wait;
//     println!("{txt}");
//     counter += 1;
//     let txt = http::Http::get("/400/HelloAsyncAwait").wait;
//     println!("{txt}");
//     counter += 1;

//     println!("Received {} responses.", counter);
// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output=String> {
    Coroutine0::new()
}

enum State0 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

/// 增加一个堆栈，保存 coroutine 块中的 counter 变量，
/// 如果有第 2 个需要保存、恢复的变量，则会再增加一个字段，以此类推。
#[derive(Default)]
struct Stack0 {
    counter: Option<usize>,
}

struct Coroutine0 {
    // 协程增加一个堆栈，用以保存状态之间需要保存、恢复的变量
    stack: Stack0,
    state: State0,
}

impl Coroutine0 {
    fn new() -> Self {
        Self { state: State0::Start, stack: Stack0::default() }
    }
}


impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        loop {
        match self.state {
                State0::Start => {
                    // initialize stack (hoist variables)
                    // 在 Start 状态的一开头就先初始化堆栈
                    self.stack.counter = Some(0);
                    // ---- Code you actually wrote ----
                    println!("Program starting");

                    // ---------------------------------
                    let fut1 = Box::new( http::Http::get("/600/HelloAsyncAwait"));
                    self.state = State0::Wait1(fut1);

                    // save stack

                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            // 进入状态一开始就从堆栈读取并恢复变量的值
                            // 注意使用了 take ，执行完堆栈的值就变成了 None
                            let mut counter = self.stack.counter.take().unwrap();

                            // ---- Code you actually wrote ----
                            println!("{txt}");
                            // 这里变量在这个状态就可以正常使用了
                            counter += 1;
                            // ---------------------------------
                            let fut2 = Box::new( http::Http::get("/400/HelloAsyncAwait"));
                            self.state = State0::Wait2(fut2);

                            // save stack
                            // 在状态末尾保存堆栈
                            self.stack.counter = Some(counter);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            let mut counter = self.stack.counter.take().unwrap();

                            // ---- Code you actually wrote ----
                            println!("{txt}");
                            counter += 1;

                            println!("Received {} responses.", counter);
                            // ---------------------------------
                            self.state = State0::Resolved;

                            // Save stack (all variables set to None already)

                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}
