#![allow(unused)]

// incremental-json parse
macro_rules! tests {
    ($($node:ident { $($json:tt)* })+) => {
        $(
            println!("{}", stringify!($node));
            $(
                tests!($json);
            )*
        )+

    };
    // ({ $($json:tt)+ }) => {};
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {}
}
