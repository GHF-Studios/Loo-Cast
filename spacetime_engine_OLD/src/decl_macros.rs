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

        TOKIO_RUNTIME.lock().unwrap().spawn(async {
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

        TOKIO_RUNTIME.lock().unwrap().block_on(async {
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

        TOKIO_RUNTIME.lock().unwrap().block_on(async {
            result = cmd(Box::new(())).await;
        });

        result
    }};
}

#[macro_export]
macro_rules! dispatch_cmds {
    (sync, [$(( $cmd_path:expr $(, $($params:expr),* )? )),* $(,)?]) => {{
        use std::pin::Pin;
        use std::future::Future;

        TOKIO_RUNTIME.lock().unwrap().block_on(async {
            let mut results = Vec::new();

            $(
                let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));
                let cmd_node = LOCKING_HIERARCHY
                    .lock()
                    .unwrap()
                    .get_node(cmd_path.clone())
                    .unwrap();

                let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
                    cmd_node.lock().unwrap().get_ref().unwrap();

                let result = cmd(Box::new(($($($params),*)?))).await;
                results.push(result);
            )*

            results
        })
    }};

    (async, batch, [$(( $cmd_path:expr $(, $($params:expr),* )? )),* $(,)?]) => {{
        use std::pin::Pin;
        use std::future::Future;

        TOKIO_RUNTIME.lock().unwrap().spawn(async {
            let mut handles = Vec::new();

            $(
                let index = handles.len();
                let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));
                let cmd_node = LOCKING_HIERARCHY
                    .lock()
                    .unwrap()
                    .get_node(cmd_path.clone())
                    .unwrap();

                let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
                    cmd_node.lock().unwrap().get_ref().unwrap();

                let handle = TOKIO_RUNTIME.lock().unwrap().spawn(async move {
                    (index, cmd(Box::new(($($($params),*)?))).await)
                });

                handles.push(handle);
            )*

            let mut results = vec![None; handles.len()];
            for handle in handles {
                if let Ok((index, result)) = handle.await {
                    results[index] = Some(result);
                }
            }

            results.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>()
        })
    }};

    (async, sequence, [$(( $cmd_path:expr $(, $($params:expr),* )? )),* $(,)?]) => {{
        use std::pin::Pin;
        use std::future::Future;

        TOKIO_RUNTIME.lock().unwrap().spawn(async {
            let mut results = Vec::new();

            $(
                let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));
                let cmd_node = LOCKING_HIERARCHY
                    .lock()
                    .unwrap()
                    .get_node(cmd_path.clone())
                    .unwrap();

                let cmd: &MutexGuard<Pin<Box<dyn Fn(Box<dyn Any>) -> Pin<Box<dyn Future<Output = Box<dyn Any>>>> + Send>>> = 
                    cmd_node.lock().unwrap().get_ref().unwrap();

                let result = cmd(Box::new(($($($params),*)?))).await;
                results.push(result);
            )*

            results
        })
    }};
}
