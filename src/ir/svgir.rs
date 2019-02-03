extern crate svg;

pub mod svgir {
    use super::svg::Node;
    use ir::common::steps::{MultipleStep, SingleStep};

    // Size of the space in pixels between text of multiple steps on the X axis.
    pub const MULTIPLE_STEP_SPACE: i32 = 100;

    pub trait StepViewer {
        fn view(&self) -> Box<svg::node::element::Group>;
    }

    impl StepViewer for SingleStep {
        fn view(&self) -> Box<svg::node::element::Group> {
            let mut g = svg::node::element::Group::new();

            let mut t = svg::node::element::Text::new();
            t.append(svg::node::Text::new(self.step.clone()));

            g.append(t);
            Box::new(g)
        }
    }

    impl StepViewer for MultipleStep {
        fn view(&self) -> Box<svg::node::element::Group> {
            let mut g = svg::node::element::Group::new();
            let mut x = 0;
            for n in &self.steps[..] {
                let mut t = svg::node::element::Text::new();
                t.append(svg::node::Text::new(*n.clone()));
                t = t.set("x", x);
                g.append(t);

                x += MULTIPLE_STEP_SPACE;
            }

            Box::new(g)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::svgir::StepViewer;
    use ir::common::steps::{MultipleStep, SingleStep};

    #[test]
    fn single_step() {
        let step = SingleStep {
            step: "test".to_string(),
        };
        let n = step.view();
        assert_eq!(n.to_string(), "<text>\ntest\n</text>");
    }

    #[test]
    fn multiple_step() {
        let mut v = Vec::new();
        v.push(Box::new("a".to_string()));
        v.push(Box::new("b".to_string()));
        let step = MultipleStep {
            steps: v,
            name: "foo".to_string(),
        };
        let n = step.view();
        assert_eq!(
            n.to_string(),
            "<g>\n<text x=\"0\">\na\n</text>\n<text x=\"100\">\nb\n</text>\n</g>"
        );
    }
}
