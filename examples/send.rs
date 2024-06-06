use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]


async fn main() {
    compile_successful().await;
    #[cfg(feature = "check")]
    compile_fail().await;
}


async fn compile_successful() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("will drop:{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

#[cfg(feature = "check")]
async fn compile_fail() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

        println!("{}", rc);
    });
}