extern crate svg as svgext;

pub mod svg {
    use super::svgext::{node, Document};
    use ir::common::steps::{MultipleStep, SingleStep, Step};
    use ir::svgir::svgir::StepViewer;
    use std::io;

    pub fn view_write_file(steps: Vec<Box<Step>>, outfn: &str) -> io::Result<()> {
        let mut document = Document::new();

        for s in steps {
            match *s {
                Step::S(s) => {
                    let n = s.view();
                    document = document.add(*n);
                }
                Step::M(m) => {
                    let n = m.view();
                    document = document.add(*n);
                }
                _ => panic!("got an invalid type"),
            }
        }

        svg::save(outfn, &document)?;
        Ok(())
    }
}
