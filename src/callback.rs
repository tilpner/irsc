pub struct Callback<A: Sized> {
    items: Vec<fn(&mut A)>
}

impl<A: Sized> Callback<A> {
    pub fn new() -> Callback<A> {
        Callback { items: Vec::new() }
    }
    pub fn register(&mut self, f: fn(&mut A)) {
        self.items.push(f)
    }
    pub fn fire(&self, v: &mut A) {
        for _ in self.items.iter().map(|&c| c(v)) {}
    }
}
