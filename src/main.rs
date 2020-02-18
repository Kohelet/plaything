#![feature(impl_trait_in_bindings)]
use clap::{Arg,App};

trait Trait {}

impl Trait for u32 {}

fn garage() -> impl Trait {
	5
}

fn main() {

	let args = App::new("plaything")
		.version("1.0.0")
		.author("kohelet <kohelet@cyclonecobra.com")
		.about("Testing command line argument parsing--for now")
		.arg(Arg::with_name("item")
				.short("i")
				.long("item")
				.takes_value(true)
				.required(false)
				.help("Random item!")).get_matches();

	let item  = args.value_of("item").unwrap().to_string();

	println!("Item is {}", item);
    println!("Hello, world!");
    let x: impl Trait = garage();
}
