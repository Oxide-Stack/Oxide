use tokio_stream::StreamExt;

#[tokio::test]
async fn watch_receiver_to_stream_emits_updates() {
    let (tx, rx) = tokio::sync::watch::channel(1_u32);
    let mut stream = crate::watch_receiver_to_stream(rx);

    let first = stream.next().await.unwrap();
    assert_eq!(first, 1);

    tx.send(2).unwrap();
    let second = stream.next().await.unwrap();
    assert_eq!(second, 2);
}

#[cfg(feature = "frb-spawn")]
#[tokio::test]
async fn runtime_spawn_executes_and_returns_result() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    let _ = crate::runtime::init(thread_pool);

    let handle = crate::runtime::spawn(async move { 123u32 });
    let result = handle.await.unwrap();
    assert_eq!(result, 123u32);
}

#[cfg(feature = "frb-spawn")]
#[tokio::test]
async fn runtime_spawn_blocking_executes_and_returns_result() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    let _ = crate::runtime::init(thread_pool);

    let handle = crate::runtime::spawn_blocking(|| 7u32);
    let result = handle.await.unwrap();
    assert_eq!(result, 7u32);
}

#[tokio::test]
async fn runtime_init_and_spawn_paths_exercised() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    let _ = crate::runtime::init(thread_pool);
    let _ = crate::init_engine_globals();
    let _ = crate::init_from_frb(thread_pool);

    let _tp = crate::runtime::thread_pool().expect("thread pool present");

    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    crate::runtime::safe_spawn(async move {
        ran_clone.store(true, Ordering::SeqCst);
    });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert!(ran.load(Ordering::SeqCst));
}

