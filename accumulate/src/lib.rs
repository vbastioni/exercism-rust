pub fn map<T, U>(input: Vec<T>, mut fun: impl FnMut(T) -> U) -> Vec<U> {
    let mut v = Vec::with_capacity(input.len());
    for e in input {
        v.push(fun(e));
    }
    v
}
