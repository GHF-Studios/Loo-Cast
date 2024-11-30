#[macro_export]
macro_rules! dispatch_cmd {
    (async, $cmd_path:expr, $params:expr) => {{
        use std::pin::Pin;
        use std::future::Future;

        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));

        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();

        let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
            cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();

        let runtime = TOKIO_RUNTIME.lock().unwrap();
        runtime.spawn(async {
            cmd(Box::new($params)).await;
        })
    }};

    (async, $cmd_path:expr) => {{
        use std::pin::Pin;
        use std::future::Future;

        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));

        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();

        let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
            cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();

        let runtime = TOKIO_RUNTIME.lock().unwrap();
        runtime.spawn(async {
            cmd(Box::new(())).await;
        })
    }};

    (sync, $cmd_path:expr, $params:expr) => {{
        use std::pin::Pin;
        use std::future::Future;

        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));

        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();

        let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
            cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();

        let result;
        let runtime = TOKIO_RUNTIME.lock().unwrap();
        runtime.block_on(async {
            result = cmd(Box::new($params)).await;
        });

        result
    }};

    (sync, $cmd_path:expr) => {{
        use std::pin::Pin;
        use std::future::Future;

        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));

        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();

        let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
            cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();

        let result;
        let runtime = TOKIO_RUNTIME.lock().unwrap();
        runtime.block_on(async {
            result = cmd(Box::new(())).await;
        });

        result
    }};
}
