#[macro_export]
macro_rules! map {
    ( <$kt:ty, $vt:ty> ) => {
        HashMap::<$kt, $vt>::new()
    };
    ( <$kt:ty, $vt:ty>; $( $k:expr => $v:expr ), + $(,)? ) => {
        {
            let mut temp_map: HashMap<$kt, $vt> = HashMap::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
    () => {
        HashMap::<_, _>::new()
    };
    ( $( $k:expr => $v:expr ), + $(,)? ) => {
        {
            let mut temp_map: HashMap<_, _> = HashMap::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn macro_generates_supposed_map() {
        assert_eq!(map!(<i8, i8>), HashMap::new());
        assert_eq!(
            map!(<i8, i8>; 1 => 1, 2 => 2, 3 => 3),
            {
                let mut map = HashMap::new();
                map.insert(1, 1);
                map.insert(2, 2);
                map.insert(3, 3);
                map
            }
        );
    }
}
