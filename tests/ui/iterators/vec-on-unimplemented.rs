//@ run-rustfix
fn main() {
    let _ = vec![true, false].map(|v| !v).collect::<Vec<_>>();
    //~^ ERROR no method named `map` found for struct `Vec<bool>` in the current scope
}
