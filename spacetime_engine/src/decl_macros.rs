#[macro_export]
macro_rules! dispatch_cmd {
    ($cmd_path:expr) => {{
        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));
        
        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();
        
        let cmd: &MutexGuard<Pin<Box<dyn Future<Output = Box<dyn Any>>>>> = cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();
        
        let runtime = TOKIO_RUNTIME.lock().unwrap();
        
        runtime.spawn(async {
            cmd().await;
        })
    }};
}

#[macro_export]
macro_rules! dispatch_cmd_blocking {
    ($cmd_path:expr) => {{
        let cmd_path = AbsoluteLockingPath::new_from_literal(stringify!($cmd_path));
        
        let cmd_node = LOCKING_HIERARCHY
            .lock()
            .unwrap()
            .get_node(cmd_path.clone())
            .unwrap();
        
        let cmd: &MutexGuard<Pin<Box<dyn Future<Output = Box<dyn Any>>>>> = cmd_node
            .lock()
            .unwrap()
            .get_ref()
            .unwrap();
        
        let runtime = TOKIO_RUNTIME.lock().unwrap();
        
        let result: Box<dyn Any>;

        runtime.block_on(async {
            result = cmd().await;
        });

        result
    }};
}