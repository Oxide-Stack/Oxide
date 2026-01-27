use std::sync::OnceLock;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static RUNTIME_DRIVER: OnceLock<()> = OnceLock::new();

pub fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build ticker_app Tokio runtime")
    })
}

pub fn init() {
    RUNTIME_DRIVER.get_or_init(|| {
        let rt = runtime();
        let _ = std::thread::Builder::new()
            .name("ticker_app_tokio_runtime".to_string())
            .spawn(move || {
                rt.block_on(std::future::pending::<()>());
            });
    });
}

pub fn handle() -> tokio::runtime::Handle {
    init();
    runtime().handle().clone()
}
