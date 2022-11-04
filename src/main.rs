use std::{
    sync::{Arc, RwLock},
    thread, time::Instant, cmp::min
};

pub fn main() {
    let now = Instant::now();
    let primes = threaded_segmented_sieve_of_eratosthenes(10000000000);
    let finished =now.elapsed().as_secs_f64();
    
    //println!("{:?}\n", primes);
    println!("found {} primes in {}s", primes.len(), finished);
}

fn threaded_segmented_sieve_of_eratosthenes(limit:usize) -> Vec<usize> {
    let threads = num_cpus::get();
    explicit_threaded_segmented_sieve_of_eratosthenes(limit, threads)
}

fn explicit_threaded_segmented_sieve_of_eratosthenes(limit:usize, threads:usize) -> Vec<usize> {
    let sqrt_of_limit = (limit as f64).sqrt().ceil() as usize;
    let early_primes = if limit <= 230 {
        Arc::new(RwLock::new(vec![2,3,5,7,11,13,17]))
    } else {
        Arc::new(RwLock::new(threaded_segmented_sieve_of_eratosthenes(sqrt_of_limit)))
    };

    let mut thread_handles = Vec::new();

    let thread_spacing = (limit - sqrt_of_limit) / threads;
    let segment_size = min(100000, sqrt_of_limit);

    for i in 0..threads {
        let early_primes = early_primes.clone();
        let lowest_checked = sqrt_of_limit + i *thread_spacing;
        let mut highest_checked = lowest_checked + thread_spacing;

        if i == threads - 1 {
            highest_checked = limit;
        }
        
        thread_handles.push(thread::spawn(move|| {
            eratosthenes_segment_thread(early_primes, lowest_checked, highest_checked, segment_size)
        }));
    }

    let mut new_primes = Vec::new();

    for handle in thread_handles {
        new_primes.append(&mut handle.join().unwrap());
    }

    let mut early_primes = early_primes.write().unwrap();
    early_primes.append(&mut new_primes);
    return early_primes.to_owned();
}

fn eratosthenes_segment_thread(early_primes:Arc<RwLock<Vec<usize>>>, lowest_checked: usize, highest_checked: usize, segment_size: usize) -> Vec<usize> {
    let mut returned_primes = Vec::new();

    let mut lower = lowest_checked;
    let mut higher = lowest_checked + segment_size;

    let early_primes = early_primes.read().unwrap();

    while lower < highest_checked {
        if higher > highest_checked {
            higher = highest_checked;
        }
        
        let mut new_primes = vec![true; segment_size];

        for i in 0..early_primes.len() {
            let mut lolim = (lower / early_primes[i]) * early_primes[i];
            if lolim < lower {
                lolim += early_primes[i]
            }

            let mut j = lolim;
            while j < higher {
                new_primes[j - lower] = false;
                j += early_primes[i];
            }
        }
        let mut p = lower;
        while p < higher {
            if new_primes[p - lower] {
                returned_primes.push(p);
            }
            p += 1
        }
        
        lower += segment_size;
        higher += segment_size;
    }
    return returned_primes;
}