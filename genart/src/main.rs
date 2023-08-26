#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

// Cross-channel usage
use crate::ConnectvityCheck::*;

// fn main() {
//   let (tx, rx) = unbounded();
//
//   thread::spawn(move || {
//     tx.send(42).unwrap();
//   });
//
//   select!{
//     recv(rx) -> msg => {
//       println!("Received {}", msg.unwrap());
//     }
//   }
// }

#[derive(Debug)]
enum ConnectvityCheck {
  Ping,
  Pong,
  Pang,
}

fn main() {
  let n_messages = 3;
  let (requests_tx, requests_rx) = unbounded();
  let (responses_tx, responses_rx) = unbounded();

  thread::span(move || loop {
    match requests_rx.recv().unwrap() {
      Pong => eprintln!("Unexpected pong response"),
      Ping => responses_tx.send(Pong).unwrap(),
      Pang => return,
    }
  });

  for _ in 0..n_messages {
    requests_tx.send(Ping).unwrap();
  }

  requests_tx.send(Pang).unwrap();

  for _ in 0..n_messages {
    select! {
        recv(responses_rx) -> msg => {
            println!("Received {:?}", msg.unwrap());
        }
    }
  }

}
