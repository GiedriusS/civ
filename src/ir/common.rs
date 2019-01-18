pub mod steps {
    pub struct SingleStep<'a> {
        pub name: &'a str,
    }

    pub struct MultipleStep<'a> {
        pub name: &'a [&'a str],
    }
}
