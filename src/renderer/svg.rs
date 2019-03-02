extern crate svg as svgext;

pub mod svg {
    use super::svgext::node::element::SVG;
    use super::svgext::Document;
    use ir::common::steps::Step;
    use ir::svgir::svgir::StepViewer;
    use std::io;

    const ADJUST_X_PER_STEP: usize = 50;

    fn view_file(steps: Vec<Box<Step>>) -> SVG {
        let mut document = Document::new();
        let mut cur_x = 0;

        document = document.set(
            "viewBox",
            format!(
                "{0} -15 {1} {1}",
                ADJUST_X_PER_STEP - 15,
                steps.len() * ADJUST_X_PER_STEP
            ),
        );

        let mut i = 0;
        while i < steps.len() {
            let s = &*steps[i];

            match s {
                Step::S(s) => {
                    let n = s.view();
                    let adjusted_node = n.set("transform", format!("translate ({}, 0)", cur_x));
                    document = document.add(adjusted_node);
                }
                Step::M(m) => {
                    let n = m.view();
                    let adjusted_node = n.set("transform", format!("translate ({}, 0)", cur_x));
                    document = document.add(adjusted_node);
                }
            }

            if i != (steps.len() - 1) {
                let mut l = svg::node::element::Line::new();
                l = l.set("stroke", "black");
                l = l.set("stroke-dasharray", "4");
                l = l.set("x1", cur_x + 30);
                l = l.set("x2", cur_x + ADJUST_X_PER_STEP);
                l = l.set("y1", -5);
                l = l.set("y2", -5);
                document = document.add(l);
            }

            cur_x += ADJUST_X_PER_STEP;
            i = i + 1;
        }

        document
    }

    pub fn view_write_file(steps: Vec<Box<Step>>, outfn: &str) -> io::Result<()> {
        let d = view_file(steps);
        svg::save(outfn, &d)?;
        Ok(())
    }
}
