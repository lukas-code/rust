pub trait Gen<'source> {
    type Output;

    fn gen<T>(&self) -> T
    where
        Self: for<'s> Gen<'s, Output = T>;
}

fn gen<'source, G, T>(g: &G) -> T
where
    G: Gen<'source>,
    for<'s> G: Gen<'s, Output = T>,
{
    <G as Gen<'source>>::gen(g)
    //~^ ERROR type annotations needed: cannot satisfy `G: Gen<'source>`
}

fn main() {}
