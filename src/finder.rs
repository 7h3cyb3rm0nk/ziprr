use std::path::Path;
use crossbeam_channel::{Sender, Receiver};

pub use crate::reader::start_password_reader;
pub use crate::worker::password_checker;
pub fn password_finder(zip_path: &str, password_list_path: &str, workers: usize) {
    let zip_file_path = Path::new(zip_path);
    let password_list_file_path = Path::new(password_list_path).to_path_buf();

    //channel with backpressure 
    //
    let (send_password, receive_password): (Sender<String>, Receiver<String>) = 
        crossbeam_channel::bounded(workers * 10_000);

    //thread handle for password reader
    //
    let password_gen_handle = start_password_reader(password_list_file_path, send_password);

    // save all worker handles
    //
    let mut worker_handles = Vec::with_capacity(workers);
    for i in 1..=workers {
        let join_handle = password_checker(i, zip_file_path, receive_password.clone());
        worker_handles.push(join_handle);
    }


    //wait for the workers to finish
    //
    for h in worker_handles {
        h.join().unwrap();
    }

    password_gen_handle.join().unwrap();
}
