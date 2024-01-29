use crossbeam_channel::Sender;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;
use std::thread::JoinHandle;

pub fn password_checker(
    index: usize,
    file_path: &Path,
    receive_password: Receiver<String>,
    ) -> JoinHandle<()> {

    let file = File::open(file_path).expect("file should exist");
    thread::Builder::new()
        .name( format!("worker-{}", index))
        .spawn( || {
            let mut archive = zip::ZipArchive::new(file).expect("Archive validated before-hand");
            loop {
                match receive_password.recv() {
                    Err(_) => break,
                    Ok(password) => {
                        let res = archive.by_index_decrypt(0, password.as_bytes());
                        match res {
                            Err(e) => panic!("unexpected error {:?}", e),
                            Ok(Err(_)) => (),
                            Ok(Ok(_)) =>  {
                                println!("password found: {}", password);
                                break;
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
}

