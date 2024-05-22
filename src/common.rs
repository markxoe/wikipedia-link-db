use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file_async<R, C>(
    file: String,
    threads: i32,
    line_handler: fn(String, C) -> Vec<R>,
    context: C,
) -> Vec<R>
where
    R: Send,
    R: Sized,
    C: Clone,
    C: Send,
{
    let (tx, rx) = crossbeam_channel::bounded(0);

    let reader_thread = std::thread::spawn(move || {
        let file_ptr = File::open(file).expect("Unable to open file");
        let reader = BufReader::new(file_ptr);

        let mut i = 0;

        for line in reader.lines().into_iter() {
            let line = line.unwrap();
            tx.send(line).expect("Error sending line");

            if i % 100 == 0 {
                debug!("Read {} lines", i);
            }
            i += 1;
        }
    });

    let output = std::thread::scope(|s| {
        let mut thread_handles = vec![];

        for _ in 0..(threads - 1) {
            let rx = rx.clone();
            let context = context.clone();

            let thread = s.spawn(move || {
                let mut out = vec![];

                let mut i = 0;

                while let Ok(res) = rx.recv() {
                    let res = line_handler(res, context.clone());
                    out.extend(res);

                    if i % 100 == 0 {
                        debug!("Parsed {} lines", i);
                    }
                    i += 1;
                }

                out
            });

            thread_handles.push(thread);
        }

        let mut outputs = vec![];
        for handle in thread_handles {
            outputs.extend(handle.join().expect("Error joining thread"));
        }

        outputs
    });

    reader_thread.join().expect("Error joining thread");

    output
}
