pub mod steps {
    #[derive(Debug)]
    pub struct SingleStep {
        pub step: String,
    }

    #[derive(Debug)]
    pub struct MultipleStep {
        pub steps: Vec<Box<String>>,
        pub name: String, // Not used at the moment
    }

    #[derive(Debug)]
    pub enum Step {
        S(SingleStep),
        M(MultipleStep),
    }
}
