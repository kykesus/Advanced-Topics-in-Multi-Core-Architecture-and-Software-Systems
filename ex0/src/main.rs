use clap::{App, Arg};
use std::thread;
use std::{cmp::min, sync::mpsc};
use std::{fs::File, io::BufRead, time::Instant};
use std::{
    io::{BufReader, Write},
    vec,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, sender: mpsc::Sender<Vec<u64>>, data: Vec<u64>) -> Worker {
        let thread = thread::spawn(move || {
            println!("Start worker {}", id);
            let sorted = recursive_mergesort(data);
            println!("Worker {} done", id);
            sender.send(sorted);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let matches = App::new("Multi-threaded Sorting")
        .version("0.1.0")
        .about("Sorts 64-bit numbers on parallel")
        .arg(
            Arg::with_name("threads")
                .takes_value(true)
                .help("Number of threads to use"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("Input file with numbers to sort"),
        )
        .get_matches();

    // Read user input
    let filename = matches.value_of("file").unwrap_or("./input.txt");
    let threads_num = match matches.value_of("threads") {
        None => 1,
        Some(n) => match n.parse::<usize>() {
            Ok(s) => s,
            Err(_) => 1,
        },
    };
    println!("{}", filename);
    println!("{}", threads_num);

    // Read data to sort
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect();

    let start = Instant::now();

    // Recursive sort
    // let sorted = recursive_mergesort(lines);

    // Parallel sort
    let sorted = parallel_mergesort(lines, threads_num);

    // Print running time
    println!("MergeSort: {:?}", start.elapsed().as_micros());

    // Print sorted output
    // println!("{:?}", sorted);

    // Write output to file
    let output = File::create("./output.txt").unwrap();
    for num in sorted {
        writeln!(&output, "{}", num).expect("fail to write data");
    }
}

fn parallel_mergesort(data: Vec<u64>, threads_num: usize) -> Vec<u64> {
    let mut workers = Vec::with_capacity(threads_num);
    let mut sub_data = Vec::with_capacity(threads_num);
    let mut receivers = Vec::with_capacity(threads_num);

    let mut chunk_size = data.len() / threads_num;
    if data.len() % threads_num > 0 {
        chunk_size += 1
    };

    println!("chunk size - {}", chunk_size);

    // Assign sub data to sort for each worker
    for id in 0..threads_num {
        let (job_tx, job_rx) = mpsc::channel();
        receivers.push(job_rx);
        let lower_idx = id * chunk_size;
        let upper_idx = min((id + 1) * chunk_size, data.len());
        println!("Sort indices {}-{}", lower_idx, upper_idx - 1);
        workers.push(Worker::new(id, job_tx, data[lower_idx..upper_idx].to_vec()));
    }

    // Get sorted data from workers
    for id in 0..threads_num {
        sub_data.push(receivers[id].recv().unwrap());
        println!("Received data from worker {}", id);
    }

    // Merge sorted sub data
    let sorted = merge_vectors(sub_data);

    sorted
}

fn recursive_mergesort(data: Vec<u64>) -> Vec<u64> {
    let n = data.len();
    let m = n / 2;
    if n <= 1 {
        return data;
    }

    let vec1 = recursive_mergesort(data[0..m].to_vec());
    let vec2 = recursive_mergesort(data[m..n].to_vec());

    let sorted = merge(vec1, vec2);
    sorted
}

fn merge(vec1: Vec<u64>, vec2: Vec<u64>) -> Vec<u64> {
    let mut merged = Vec::new();

    let mut i = 0;
    let mut j = 0;
    while i < vec1.len() && j < vec2.len() {
        if vec1[i] < vec2[j] {
            merged.push(vec1[i]);
            i += 1;
        } else {
            merged.push(vec2[j]);
            j += 1;
        }
    }
    if i < vec1.len() {
        merged.extend_from_slice(&vec1[i..]);
    } else {
        merged.extend_from_slice(&vec2[j..]);
    }
    assert_eq!(merged.len(), vec1.len() + vec2.len());
    merged
}

fn merge_vectors(vectors: Vec<Vec<u64>>) -> Vec<u64> {
    let mut sorted = Vec::new();

    for i in 0..vectors.len() {
        sorted = merge(sorted, vectors[i].clone());
    }

    sorted
}

#[test]
fn test_merge() {
    let v1: Vec<u64> = vec![1, 2, 5, 7, 11];
    let v2: Vec<u64> = vec![2, 3, 4, 5, 8, 9, 14];
    let expected: Vec<u64> = vec![1, 2, 2, 3, 4, 5, 5, 7, 8, 9, 11, 14];

    let merged = merge(v1, v2);
    assert_eq!(merged, expected);
}

#[test]
fn test_recursive_mergesort() {
    let v1: Vec<u64> = vec![
        5, 1, 67, 2345, 875, 235, 573568, 8967, 325, 78, 252, 1, 8, 579, 324, 80,
    ];
    let v2: Vec<u64> = vec![
        1, 1, 5, 8, 67, 78, 80, 235, 252, 324, 325, 579, 875, 2345, 8967, 573568,
    ];

    let sorted = recursive_mergesort(v1);
    assert_eq!(sorted, v2);
}
