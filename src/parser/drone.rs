extern crate serde_yaml;

mod parser {
    use super::serde_yaml::{Mapping, Value};
    use ir::common::steps::{MultipleStep, SingleStep, Step};

    pub fn from_string(input: &str) -> Result<Vec<Box<Step>>, &'static str> {
        let obj: Value = serde_yaml::from_str(input).unwrap();

        let pipeline = &obj["pipeline"];
        let topmapping = match pipeline {
            Value::Mapping(m) => m,
            _ => return Err("top level pipeline mapping not found"),
        };

        let mut ret: Vec<Box<Step>> = Vec::new();

        for data in topmapping.iter() {
            let (pk, pd) = data;

            let ppk = pk.as_str().ok_or("mapping key is not a string")?;
            //let ppd = pd.as_mapping().ok_or("sub value is not a mapping")?;

            let s = Box::new(Step::S(SingleStep {
                name: ppk.to_string(),
            }));
            ret.push(s);
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    fn base_case() {
        let base = r#"
---
pipeline:
  step1:
    command: echo hi

  step2:
    command: echo bye
"#;
        let result = parser::from_string(&base);

        match result {
            Ok(res) => {
                assert_eq!(res.len(), 2);
            }
            Err(e) => panic!("error occurred: {}", e),
        }
    }
}
