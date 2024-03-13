
fn fmap<A, B, F> (f: F, a: A) -> B
    where F: Fn(A) -> B
{
    f(a)
}

fn bimap<A, B, C, D, F, G> (f: F, g: G, a: A, b: B) -> (C, D)
    where F: Fn(A) -> C,
          G: Fn(B) -> D
{
    (f(a), g(b))
}

fn main() {
    let f = |x| x + 1;
    let a = 1;
    let b = fmap(f, a);
    println!("{}", b);

    let g = |x| x + 2;
    let c = 2;
    let d = 3;
    let (e, h) = bimap(f, g, c, d);
    println!("{} {}", e, h);
}
