use log::debug;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::indication;

pub fn get_file_line_count(file: &str) -> u64 {
    let spinner = indication::spinner(true); // with progress, should fit the other progress bars
    spinner.set_message(format!("Loading line count for {file}"));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let file_ptr = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file_ptr);

    let count = reader.lines().count() as u64;

    spinner.finish_and_clear();

    count
}

pub fn parse_file_async<R, C>(
    file: String,
    threads: i32,
    line_handler: fn(String, C) -> Vec<R>,
    context: C,
) -> VecDeque<R>
where
    R: Send,
    R: Sized,
    C: Clone,
    C: Send,
{
    if threads < 2 {
        panic!("Threads must be greater than or equal 2");
    }

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

        let mut outputs = VecDeque::new();
        for handle in thread_handles {
            outputs.extend(handle.join().expect("Error joining thread"));
        }

        outputs
    });

    reader_thread.join().expect("Error joining thread");

    output
}

mod test {
    #[allow(unused_imports)]
    use std::{
        env::temp_dir,
        io::Write,
        sync::{Arc, Mutex},
    };

    #[test]
    fn all_lines_are_read() {
        let dir = temp_dir();
        let file_path = dir.join("test.txt");

        // create file
        {
            let file = std::fs::File::create(&file_path).expect("Unable to create file");
            let mut writer = std::io::BufWriter::new(file);
            for i in 0..1000 {
                writeln!(writer, "{}", i).expect("Unable to write to file");
            }
        }

        let call_count = Arc::new(Mutex::new(0));

        // parse file
        let result = super::parse_file_async(
            file_path.to_str().unwrap().to_string(),
            2,
            |line, ctx| {
                *ctx.lock().unwrap() += 1;
                vec![line.parse::<i32>().unwrap()]
            },
            call_count.clone(),
        );

        // check that all lines are read
        assert_eq!(result.len(), 1000);
        for i in 0..1000 {
            assert!(result.contains(&i));
        }

        // check parser call count
        assert_eq!(*call_count.lock().unwrap(), 1000);
    }
}
