pub mod steps {
    #[derive(Debug, PartialEq)]
    pub struct SingleStep {
        pub step: String,
    }

    #[derive(Debug, PartialEq)]
    pub struct MultipleStep {
        pub steps: Vec<Box<String>>,
        pub name: String, // Not used at the moment
    }

    #[derive(Debug, PartialEq)]
    pub enum Step {
        S(SingleStep),
        M(MultipleStep),
    }
}
