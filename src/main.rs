fn do_ref (x:&str) {
    println!("{}", x);
}

#[derive(Debug)]
enum SocketState {
    Open,
    Stream
}

struct Dlaw {
    width:  u32,
    name: &'static str
}

trait Treats {
    fn new (name: &'static str) -> Self;

    fn calc (&self) -> u32;
}

impl Dlaw {
    fn check (&self) -> u32 {
        self.width * 2
    }
}

impl Treats for Dlaw {
    fn new (name: &'static str) -> Dlaw {
        Dlaw { name: name, width: 10 }
    }

    fn calc (&self) -> u32 {
        self.width * 10
    }
}

fn get_value (state: SocketState) -> u32 {
    match state {
        SocketState::Open => { 2 + 2 },
        SocketState::Stream => 1
    }
}

static LANG: &'static str = "lkajskldjaskld";

fn main() {
    let closure_annotated = |&i| -> i32 { i + 1 };

    let q = [1,5,10,15,20];
    for y in q[2..4].iter() {
        println!("Foo {}", closure_annotated(y));
    }

    do_ref("foo");
    println!("State {}", get_value(SocketState::Open));

    let d = Dlaw { name: LANG, width: 10 };
    println!("Dlaw check {}", d.check());
}
