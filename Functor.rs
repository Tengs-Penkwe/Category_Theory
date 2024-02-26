
fn fmap<A, B, F> (f: F, a: A) -> B
    where F: Fn(A) -> B
{
    f(a)
}

fn main() {
    let f = |x| x + 1;
    let a = 1;
    let b = fmap(f, a);
    println!("{}", b);
}
