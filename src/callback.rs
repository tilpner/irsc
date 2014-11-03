pub struct Callback<A: Sized + Send> {
    items: Vec<fn(A)>
}

impl<A: Sized + Clone + Send> Callback<A> {
    pub fn new() -> Callback<A> {
        Callback { items: Vec::new() }
    }
    pub fn register(&mut self, f: &fn(A)) {
        self.items.push(*f)
    }
    pub fn fire(&self, v: &A) {
        for _ in self.items.iter().map(|&c| c(v.clone())) {}
    }
}
