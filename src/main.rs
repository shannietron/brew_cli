extern crate hyper;
use hyper::client::Client;
use hyper::client::Response;
use std::io::Read;

fn main() {
	let client = Client::new();
	let mut res: Response = client.get("https://api.thingspeak.com/channels/98643/feed/last.json").send().unwrap();
			assert_eq!(res.status, hyper::Ok);
	let mut buffer = String::new();
	res.read_to_string(&mut buffer);

	println!("{}",res.url);
	println!("{}",buffer);
}
