extern crate spacetime_engine;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_rapier2d::prelude::*;
use spacetime_engine::*;
use spacetime_engine::core::singletons::*;

// Primary tasks
// TODO: Implement chunk loaders

// Secondary tasks
// TODO: Implement default for all registries and registry wrappers instead of the new function 

// Fun tasks
// TODO: Implement inventory + hotbar, so that you can select different types of chunk actors to place. 

// Less fun tasks
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

//fn main() {
//    //env::set_var("RUST_BACKTRACE", "1");
//    
//    App::new()
//        .add_plugins(DefaultPlugins.set(LogPlugin {
//            filter: "info,spacetime_engine=debug".into(),
//            level: bevy::log::Level::INFO,
//            ..Default::default()
//        }))
//        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
//        .add_plugins(SpacetimeEnginePlugins)
//        .add_systems(PreStartup, pre_startup)
//        .add_systems(Startup, startup)
//        .run();
//}

fn pre_startup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}

fn startup() {
    let runtime = TOKIO_RUNTIME.lock().unwrap();

    runtime.spawn(async {
        spacetime_engine::core::commands::startup().await;
    });
}







use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use spacetime_engine::core::structs::*;

/// Test 1: Basic Map Locking
/// This test checks if we can lock the entire map and if individual entries are blocked while the map is locked.
fn test_basic_map_locking() {
    println!("Test 1: Basic Map Locking");

    let map = HierarchicalLockingMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    // Lock the entire map
    if let Some(map_handle) = map.lock_map() {
        let map_guard = map_handle.lock();
        println!("Map locked, contents: {:?}", map_guard);

        // Try to lock an entry while the map is locked (should fail)
        if map.lock_entry("key1".to_string()).is_none() {
            println!("Correctly prevented entry lock while map is locked.");
        } else {
            panic!("Failed: Entry lock should not have been allowed while map is locked.");
        }
    } else {
        panic!("Failed: Unable to lock map.");
    }
}

/// Test 2: Basic Entry Locking
/// This test ensures that locking individual entries works and prevents map-level locks while an entry is locked.
fn test_basic_entry_locking() {
    println!("Test 2: Basic Entry Locking");

    let map = HierarchicalLockingMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    // Lock entry 'key1'
    if let Some(entry_handle) = map.lock_entry("key1".to_string()) {
        let mut entry = entry_handle.lock();
        println!("Entry 'key1' locked, value: {:?}", *entry);
        *entry += 5; // Modify the entry
        println!("Modified 'key1' value: {:?}", *entry);

        // Try to lock the entire map while 'key1' is locked (should fail)
        if map.lock_map().is_none() {
            println!("Correctly prevented map lock while entry is locked.");
        } else {
            panic!("Failed: Map lock should not have been allowed while entry is locked.");
        }
    } else {
        panic!("Failed: Unable to lock entry 'key1'.");
    }
}

/// Test 3: Entry-Map Conflict Test
/// This test ensures that if either the map or an entry is locked, the other cannot be locked concurrently.
fn test_entry_map_conflict() {
    println!("Test 3: Entry-Map Conflict Test");

    let map = HierarchicalLockingMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    // Lock entry 'key1'
    if let Some(entry_handle) = map.lock_entry("key1".to_string()) {
        let _entry = entry_handle.lock();
        println!("Entry 'key1' is locked.");

        // Try to lock the map (should fail)
        if map.lock_map().is_none() {
            println!("Correctly prevented map lock while 'key1' is locked.");
        } else {
            panic!("Failed: Map lock should not have been allowed while 'key1' is locked.");
        }
    } else {
        panic!("Failed: Unable to lock entry 'key1'.");
    }

    // Lock the entire map
    if let Some(map_handle) = map.lock_map() {
        let _map_guard = map_handle.lock();
        println!("Map is locked.");

        // Try to lock 'key1' (should fail)
        if map.lock_entry("key1".to_string()).is_none() {
            println!("Correctly prevented entry lock while the map is locked.");
        } else {
            panic!("Failed: Entry lock should not have been allowed while map is locked.");
        }
    } else {
        panic!("Failed: Unable to lock map.");
    }
}

/// Test 4: Concurrent Entry Locking
/// This test checks concurrent locking of different entries to ensure they can be modified independently.
fn test_concurrent_entry_locking() {
    println!("Test 4: Concurrent Entry Locking");

    let map = Arc::new(HierarchicalLockingMap::new());
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    let map_clone1 = Arc::clone(&map);
    let map_clone2 = Arc::clone(&map);

    let handle1 = thread::spawn(move || {
        if let Some(entry_handle) = map_clone1.lock_entry("key1".to_string()) {
            let mut entry = entry_handle.lock();
            println!("Thread 1 locked 'key1', value: {:?}", *entry);
            *entry += 10; // Modify the entry
            println!("Thread 1 modified 'key1' value: {:?}", *entry);
        } else {
            panic!("Thread 1 failed to lock 'key1'.");
        }
    });

    let handle2 = thread::spawn(move || {
        if let Some(entry_handle) = map_clone2.lock_entry("key2".to_string()) {
            let mut entry = entry_handle.lock();
            println!("Thread 2 locked 'key2', value: {:?}", *entry);
            *entry += 20; // Modify the entry
            println!("Thread 2 modified 'key2' value: {:?}", *entry);
        } else {
            panic!("Thread 2 failed to lock 'key2'.");
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Check if the modifications were successful
    if let Some(entry_handle) = map.lock_entry("key1".to_string()) {
        let entry = entry_handle.lock();
        assert_eq!(*entry, 20, "Expected 'key1' to have value 20 after modification.");
        println!("Test 4 passed for 'key1'.");
    }

    if let Some(entry_handle) = map.lock_entry("key2".to_string()) {
        let entry = entry_handle.lock();
        assert_eq!(*entry, 40, "Expected 'key2' to have value 40 after modification.");
        println!("Test 4 passed for 'key2'.");
    }
}

/// Test 5: Entry Lock and Modify
/// This test ensures individual entries can be locked and modified correctly, without interfering with other entries.
fn test_entry_lock_and_modify() {
    println!("Test 5: Entry Lock and Modify");

    let map = HierarchicalLockingMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    // Lock and modify 'key1'
    if let Some(entry_handle) = map.lock_entry("key1".to_string()) {
        let mut entry = entry_handle.lock();
        println!("Locked 'key1', original value: {:?}", *entry);
        *entry += 15;
        println!("Modified 'key1', new value: {:?}", *entry);
        assert_eq!(*entry, 25, "Expected 'key1' value to be 25 after modification.");
    } else {
        panic!("Failed: Unable to lock 'key1'.");
    }

    // Lock and modify 'key2'
    if let Some(entry_handle) = map.lock_entry("key2".to_string()) {
        let mut entry = entry_handle.lock();
        println!("Locked 'key2', original value: {:?}", *entry);
        *entry += 5;
        println!("Modified 'key2', new value: {:?}", *entry);
        assert_eq!(*entry, 25, "Expected 'key2' value to be 25 after modification.");
    } else {
        panic!("Failed: Unable to lock 'key2'.");
    }
}

/// Test 6: Concurrent Mixed Operations
/// This test performs a mixture of concurrent operations (map locks, entry locks, inserts)
/// from multiple threads to ensure the system holds up under high contention.
fn test_concurrent_mixed_operations() {
    println!("Test 6: Concurrent Mixed Operations");

    let map = Arc::new(HierarchicalLockingMap::new());
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);
    map.insert("key3".to_string(), 30);

    let map_clone1 = Arc::clone(&map);
    let map_clone2 = Arc::clone(&map);
    let map_clone3 = Arc::clone(&map);

    // Thread 1: Lock the map and perform an insert (hold lock for a while)
    let handle1 = thread::spawn(move || {
        if let Some(map_handle) = map_clone1.lock_map() {
            let mut map_guard = map_handle.lock();
            println!("Thread 1 locked the map.");
            map_guard.insert("key4".to_string(), Arc::new(RwLock::new(40)));
            println!("Thread 1 inserted 'key4'.");
            thread::sleep(Duration::from_millis(200)); // Hold the map lock for a while
            println!("Thread 1 releasing map lock.");
        } else {
            panic!("Thread 1 failed to lock the map.");
        }
    });

    // Thread 2: Try to lock entry 'key1' (should fail while map is locked)
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Ensure Thread 1 locks the map first
        if map_clone2.lock_entry("key1".to_string()).is_none() {
            println!("Thread 2 correctly failed to lock 'key1' while the map is locked.");
        } else {
            panic!("Thread 2 should NOT have been able to lock 'key1'.");
        }
    });

    // Thread 3: Insert a new entry after the map is unlocked
    let handle3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(300)); // Wait for Thread 1 to release the map
        map_clone3.insert("key5".to_string(), 50);
        println!("Thread 3 inserted 'key5' after the map was unlocked.");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    // Verify final state
    if let Some(map_handle) = map.lock_map() {
        let map_guard = map_handle.lock();
        assert!(map_guard.contains_key("key4"), "Expected 'key4' to be present.");
        assert!(map_guard.contains_key("key5"), "Expected 'key5' to be present.");
        println!("Test 6 passed: All expected keys present in the map.");
    }
}

/// Test 7: Sequential Conflicting Operations
/// This test forces sequences of conflicting operations (map and entry locks)
/// to check if the internal state transitions correctly.
fn test_sequential_conflicting_operations() {
    println!("Test 7: Sequential Conflicting Operations");

    let map = HierarchicalLockingMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    // Step 1: Lock entry 'key1'
    let entry_handle = map.lock_entry("key1".to_string());
    if let Some(ref entry_handle) = entry_handle {
        let _entry = entry_handle.lock();
        println!("Step 1: Locked 'key1'.");

        // Step 2: Attempt to lock the map (should fail while 'key1' is still locked)
        if map.lock_map().is_none() {
            println!("Step 2: Correctly prevented map lock while 'key1' is locked.");
        } else {
            panic!("Step 2: Should not have been able to lock the map.");
        }
    } else {
        panic!("Step 1: Failed to lock 'key1'.");
    }

    // Step 3: Unlock 'key1' and immediately lock the map
    drop(entry_handle); // Explicitly drop the entry_handle to unlock the entry
    let map_handle = map.lock_map();
    if let Some(ref map_handle) = map_handle {
        let _map_guard = map_handle.lock();
        println!("Step 3: Successfully locked the map after unlocking 'key1'.");
    } else {
        panic!("Step 3: Should have been able to lock the map.");
    }

    // Step 4: Attempt to lock an entry while the map is locked (should fail)
    if map.lock_entry("key2".to_string()).is_none() {
        println!("Step 4: Correctly prevented entry lock while the map is locked.");
    } else {
        panic!("Step 4: Should not have been able to lock an entry while the map is locked.");
    }

    // Final check: Ensure all entries are accessible after all locks are dropped
    drop(map_handle); // Explicitly drop the map_handle to unlock the map
    if let Some(entry_handle) = map.lock_entry("key2".to_string()) {
        let entry = entry_handle.lock();
        assert_eq!(*entry, 20, "Expected 'key2' to have value 20.");
        println!("Test 7 passed: Sequential operations behaved as expected.");
    }
}

/// Test 8: Long-Held Lock
/// This test holds a lock for a long time in one thread, while other threads
/// attempt to access the map and entries (they should fail until the lock is released).
fn test_long_held_lock() {
    println!("Test 8: Long-Held Lock");

    let map = Arc::new(HierarchicalLockingMap::new());
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    let map_clone1 = Arc::clone(&map);
    let map_clone2 = Arc::clone(&map);

    // Thread 1: Lock 'key1' for a long time
    let handle1 = thread::spawn(move || {
        if let Some(entry_handle) = map_clone1.lock_entry("key1".to_string()) {
            let mut entry = entry_handle.lock();
            println!("Thread 1 locked 'key1', value: {:?}", *entry);
            *entry += 10;
            println!("Thread 1 modified 'key1', new value: {:?}", *entry);
            thread::sleep(Duration::from_millis(500)); // Hold the lock for a long time
            println!("Thread 1 releasing 'key1'.");
        } else {
            panic!("Thread 1 failed to lock 'key1'.");
        }
    });

    // Thread 2: Try to lock the map (should fail until 'key1' is released)
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // Give Thread 1 a head start
        if map_clone2.lock_map().is_none() {
            println!("Thread 2 correctly failed to lock the map while 'key1' is locked.");
        } else {
            panic!("Thread 2 should not have been able to lock the map.");
        }

        // Try again after 'key1' is unlocked
        thread::sleep(Duration::from_millis(500)); // Wait for 'key1' to be unlocked
        if let Some(map_handle) = map_clone2.lock_map() {
            let _map_guard = map_handle.lock();
            println!("Thread 2 successfully locked the map after 'key1' was unlocked.");
        } else {
            panic!("Thread 2 failed to lock the map after 'key1' was unlocked.");
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    test_basic_map_locking();
    test_basic_entry_locking();
    test_entry_map_conflict();
    test_concurrent_entry_locking();
    test_entry_lock_and_modify();
    test_concurrent_mixed_operations();
    test_sequential_conflicting_operations();
    test_long_held_lock();
}