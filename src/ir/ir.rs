extern crate svg;

mod ir {
    pub trait StepViewer {
        fn view(&self) -> Box<svg::Node>;
    }

    pub struct SingleStep<'a> {
        name: &'a str
    }

    pub struct MultipleStep<'a> {
        name: [&'a str]
    }

    impl<'a> StepViewer for SingleStep<'a> {
        fn view(&self) -> Box<svg::Node> {
            return Box::new(svg::node::Text::new("stub"))
        }
    }

    impl<'a> StepViewer for MultipleStep<'a> {
        fn view(&self) -> Box<svg::Node> {
            return Box::new(svg::node::Text::new("stub"))
        }
    }
}