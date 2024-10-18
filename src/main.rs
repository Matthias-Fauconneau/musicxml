#![feature(iter_next_chunk, let_chains)]
mod measure;
mod beam;
mod attributes;
mod direction;
mod harmony;
mod layout; pub use layout::layout;

fn main() {
	std::panic::set_hook(Box::new(|p| {
		let msg =
			if let Some(s) = p.payload().downcast_ref::<String>() { s.as_str() }
			else if let Some(s) = p.payload().downcast_ref::<&str>() { s }
			else { unreachable!() };
		println!("{}:{}: {}", p.location().unwrap().file(), p.location().unwrap().line(), msg);
	}));
	let [_,path] = std::env::args().next_chunk().unwrap();
	let music = music::parse_utf8(&std::fs::read(path).unwrap()).unwrap();
    ui::run(&music.work.title, &mut ui::graphic::Widget(|size| Ok(layout(&music.part[0..2], size))))
}
