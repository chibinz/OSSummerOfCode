// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for i in 0..11 {
            thread::sleep(Duration::from_millis(250));
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        }
    });

    loop {
        thread::sleep(Duration::from_millis(500));
        let s = status.lock().unwrap();
        if s.jobs_completed < 10 {
            println!("waiting...");
        } else {
            break;
        }
    }
}