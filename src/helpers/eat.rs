pub fn eat<'i, T>(input: &mut &'i [T]) -> &'i T {
    let value = &input[0];
    *input = &input[1..];
    value
}

pub fn eat_copy<T: Copy>(input: &mut &[T]) -> T {
    *eat(input)
}

pub fn eat_while<'i, T>(input: &mut &'i [T], mut f: impl FnMut(&'i T) -> bool) -> &'i [T] {
    let mut i = 0;
    while i < input.len() && f(&input[i]) {
        i += 1;
    }
    let value = &input[..i];
    *input = &input[i..];
    value
}
