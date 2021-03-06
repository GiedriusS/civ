extern crate serde_yaml;

pub mod drone {
    use super::serde_yaml::Value;
    use ir::common::steps::{MultipleStep, SingleStep, Step};

    pub fn from_string(input: &str) -> Result<Vec<Box<Step>>, &'static str> {
        let mut ret: Vec<Box<Step>> = Vec::new();
        let obj: Value = serde_yaml::from_str(input).unwrap();

        let pipeline = &obj["pipeline"];
        let topmapping = match pipeline {
            Value::Mapping(m) => m,
            _ => return Err("top level pipeline mapping not found"),
        };

        let mut last_group_name: String = "".to_string();

        for data in topmapping.iter() {
            let (pk, pd) = data;

            let ppk = pk.as_str().ok_or("mapping key is not a string")?;
            let ppd = pd.as_mapping().ok_or("sub value is not a mapping")?;

            if ppd.contains_key(&Value::String("group".to_string())) == true {
                let ppgroup = match &ppd[&Value::String("group".to_string())] {
                    Value::String(s) => s,
                    _ => return Err("group key found but it is not a string"),
                };

                if (last_group_name != "" && last_group_name != *ppgroup) || (last_group_name == "")
                {
                    // Create a new one.
                    let mut s = Vec::new();
                    s.push(Box::new(ppk.to_string()));
                    ret.push(Box::new(Step::M(MultipleStep {
                        steps: s,
                        name: ppgroup.to_string(),
                    })));
                    last_group_name = ppgroup.to_string();
                } else {
                    // Append to the last one a new key.
                    let last_el = ret.len() - 1;
                    let mut last = &mut *ret[last_el];
                    match last {
                        Step::M(ref mut m) => {
                            m.steps.push(Box::new(ppk.to_string()));
                        }
                        _ => return Err("expected to match a multistep"),
                    }
                }
            } else {
                let s = Box::new(Step::S(SingleStep {
                    step: ppk.to_string(),
                }));
                ret.push(s);
                last_group_name = "".to_string();
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::drone;
    use ir::common::steps::Step;

    #[test]
    fn simple_case() {
        let base = r#"
---
pipeline:
  step1:
    command: echo hi

  step2:
    command: echo bye
"#;
        let result = drone::from_string(&base);

        match result {
            Ok(res) => {
                assert_eq!(res.len(), 2);
            }
            Err(e) => panic!("error occurred: {}", e),
        }
    }

    #[test]
    fn group_case() {
        let base = r#"
---
pipeline:
  step1:
    group: a
    command: echo hi

  step2:
    group: a
    command: echo bye
"#;
        let result = drone::from_string(&base);

        match result {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                match &*res[0] {
                    Step::M(m) => {
                        assert_eq!(m.steps.len(), 2);
                        assert_eq!(*m.steps[0], "step1");
                        assert_eq!(*m.steps[1], "step2");
                        assert_eq!(m.name, "a");
                    }
                    _ => panic!("got unexpected type"),
                }
            }
            Err(e) => panic!("error occurred: {}", e),
        }
    }
}
