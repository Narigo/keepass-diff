pub fn apply<A, B, C, F: Fn(A, B) -> C>(f: &'static F) -> impl Fn((A, B)) -> C {
  move |args: (A, B)| {
    let (a, b) = args;
    f(a, b)
  }
}
