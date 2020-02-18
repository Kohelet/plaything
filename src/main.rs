#![feature(impl_trait_in_bindings)]


trait Trait {
	fn show(&self);
}

impl Trait for u32 {
	fn show(&self) {
		println!("{}", &self);
	}
}

fn garage() -> Box<dyn Trait> {
	Box::new(5)
}

fn main() {

    println!("Hello, world!");
    let x: Box<dyn Trait> = garage();

    x.show();
}
