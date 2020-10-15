struct Tester {
    x: i8,
}

impl Tester {
    fn traverse<F>(&mut self, mut f: F)
        where
            F: FnMut(&mut Tester),
    {
        f(self);
    }
}

fn main() {
    let mut tester = Tester { x: 8 };
    tester.traverse(|z| z.x += 1);
    println!("{}", tester.x);
}