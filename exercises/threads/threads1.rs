// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

//// Answer 1
//use std::sync::Mutex;
//
//struct JobStatus {
//    jobs_completed: Mutex<u32>,
//}
//
//fn main() {
//    let status = Arc::new(JobStatus { jobs_completed: std::sync::Mutex::new(0) });
//    let status_shared = status.clone();
//    thread::spawn(move || {
//        for _ in 0..10 {
//            thread::sleep(Duration::from_millis(250));
//            *status_shared.jobs_completed.lock().unwrap() += 1
//        }
//    });
//    while *status.jobs_completed.lock().unwrap() < 10 {
//        println!("waiting... ");
//        thread::sleep(Duration::from_millis(500));
//    }
//}

//// Answer 2 using AtomicUsize
//use std::sync::atomic::{AtomicUsize, Ordering};
//
//struct JobStatus {
//    jobs_completed: AtomicUsize,
//}
//
//fn main() {
//    let status = Arc::new(JobStatus { jobs_completed: AtomicUsize::new(0) });
//    let status_shared = status.clone();
//    thread::spawn(move || {
//        for _ in 0..10 {
//            thread::sleep(Duration::from_millis(250));
//            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
//        }
//    });
//    while status.jobs_completed.load(Ordering::SeqCst) < 10 {
//        println!("waiting... ");
//        thread::sleep(Duration::from_millis(500));
//    }
//}

// Answer 3 using Mutex on JobStatus as the hint suggests
// This is definitely what you want if you consider there
// could be multiple fields in a struct.
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
