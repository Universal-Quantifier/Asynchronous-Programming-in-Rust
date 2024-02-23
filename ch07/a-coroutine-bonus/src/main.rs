use std::future::Future;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use dummy_waker::dummy_waker;

mod http;

async fn async_main() {
    println!("Program starting");
    let txt = http::Http::get("/600/HelloAsyncAwait1").await;
    println!("{txt}");
    let txt = http::Http::get("/400/HelloAsyncAwait2").await;
    println!("{txt}");
}

fn main() {
    let fut = async_main();
    // 和 a-coroutine 比较，主要的变化就是需要组装 context 传入 poll 方法，
    // 并且 future 对象需要包裹在 pin 里面
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut pinned = Box::pin(fut);
    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Pending => {
                println!("Schedule other tasks");
                thread::sleep(Duration::from_millis(100));
            }

            Poll::Ready(_) => break,
        }
    }
}
