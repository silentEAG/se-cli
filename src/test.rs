// incremental-json parse
#[allow(unused)]
macro_rules! tests {
    ($node:ident {
        $(
            $(#[doc = $doc:literal])+
            $name:ident : $ty:ident, $editable:literal, $none_action:ident $(, $default:expr)?;
        )+
    }) => {
        $(
            println!(concat!(stringify!($node), "-", stringify!($name)));
        )+
    };

    ($node:ident { $($son:tt)+ }) => {
        println!(stringify!($node));
        tests!($($son)+);
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        tests!(
            sakura {
                miko {
                    /// get base url
                    base_url: String, true, def, "localhost".to_string();
                    /// age
                    age: i32, true, def, 12;
                }
            }
        );
    }
}
