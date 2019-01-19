pub mod steps {
    #[derive(Debug)]
    pub struct SingleStep {
        pub name: String,
    }

    #[derive(Debug)]
    pub struct MultipleStep {
        pub name: Box<[String]>,
    }

    #[derive(Debug)]
    pub enum Step {
        S(SingleStep),
        M(MultipleStep),
    }
}
