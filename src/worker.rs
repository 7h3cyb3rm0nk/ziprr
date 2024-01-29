use crossbeam_channel::{Receiver};
use std::fs::File;
use std::io::Read;
use std::path::{Path};
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
        .spawn(  move || {
            let mut archive = zip::ZipArchive::new(file).expect("Archive validated before-hand");
            loop {
                match receive_password.recv() {
                    Err(_) => break,
                    Ok(password) => {
                        let res = archive.by_index_decrypt(0, password.as_bytes());
                        match res {
                            Err(e) => panic!("unexpected error {:?}", e),
                            Ok(Err(_)) => (),
                            Ok(Ok(mut zip)) =>  {
                                let mut buffer = Vec::with_capacity(zip.size() as usize);
                                match zip.read_to_end(&mut buffer) {
                                    Err(_) => (),
                                    Ok(_) => {
                                        println!("password found: {}", password);
                                        break; // stop thread
                                    },
                                };
                            },
                            };
                        }
                    }
                }
            }
        )
        .unwrap()
}

