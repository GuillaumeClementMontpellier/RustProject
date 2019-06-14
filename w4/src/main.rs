use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);

	let h1 = thread::spawn(move || {
		let vals = vec![
		String::from("hi"),
		String::from("from"),
		String::from("the"),
		String::from("thread"),
		];

		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	let h2 = thread::spawn(move || {
		let vals = vec![
		String::from("more"),
		String::from("messages"),
		String::from("for"),
		String::from("you"),
		];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx { //bloquant jusqu'a ce que les 2 thread se ferment (les 2 tx sont drop)
		println!("Got: {}", received);
	}

	println!("End of Recv");

	h1.join().unwrap();
	h2.join().unwrap();

}
