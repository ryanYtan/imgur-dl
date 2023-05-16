use std::collections::HashMap;
use std::marker::PhantomData;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
enum TemplateElement {
    Part {
        literal: String
    },
    Replace {
        selector: String
    },
}

pub struct Formatter {
    elements: Vec<TemplateElement>
}

impl Formatter {
    /// Rules:
    /// "..." refers to any string (including empty string) WITHOUT a closed rounded parenthesis ")"
    ///
    /// 1. "%%" is treated as a literal "%"
    /// 2. "%" followed by any character that is not "(" is invalid
    /// 3. "%(...)" is always valid
    /// 4. "%(...)x" where "x" is any string is valid
    /// 5. "%(..." is invalid
    pub fn build(fmts: &str) -> Result<Self> {
        let mut chars = fmts.chars();
        let mut result = Vec::new();
        let mut tmp = String::new();

        while let Some(c) = chars.next() {
            if c != '%' {
                tmp.push(c);
                continue;
            }

            match chars.next() {
                Some('%') => tmp.push('%'),
                Some('(') => {
                    result.push(TemplateElement::Part { literal: tmp.clone() });
                    tmp.clear();
                    let mut selector = String::new();
                    loop {
                        match chars.next() {
                            Some(')') => break,
                            Some(c) => selector.push(c),
                            None => return Err(anyhow!("unexpected end reached in template '{}'", &fmts)),
                        }
                    }
                    result.push(TemplateElement::Replace { selector });
                },
                Some(c) => return Err(anyhow!("found invalid char '{}' at position", c)),
                None => return Err(anyhow!("unexpected end reached in template '{}'", &fmts)),
            };
        }

        if !tmp.is_empty() {
            result.push(TemplateElement::Part { literal: tmp.clone() });
        }

        Ok(Self { elements: result })
    }
}

pub struct Templater<T> {
    _pd: PhantomData<T>,
    mapping: HashMap<String, Box<dyn Fn(&T) -> Option<String> + 'static + Send + Sync>>
}

impl<T> Templater<T> {
    pub fn new() -> Self {
        Templater {
            _pd: PhantomData,
            mapping: HashMap::new(),
        }
    }

    pub fn add_selector<F>(&mut self, selector: impl Into<String>, accessor: F)
            where F: Fn(&T) -> Option<String> + 'static + Send + Sync
    {
        self.mapping.insert(selector.into(), Box::new(accessor));
    }

    #[allow(dead_code)]
    pub fn add_selectors<I, S, U>(&mut self, selectors: I)
        where
            I: IntoIterator<Item = (S, U)>,
            S: Into<String>,
            U: (Fn(&T) -> Option<String>) + 'static + Send + Sync
    {
        for (selector, accessor) in selectors {
            self.add_selector(selector, accessor);
        }
    }

    pub fn render(&self, obj: &T, formatter: &Formatter) -> Result<String> {
        let mut output = String::new();
        for element in formatter.elements.iter() {
            match element {
                TemplateElement::Part { literal } => {
                    output.push_str(&literal);
                },
                TemplateElement::Replace { selector } => {
                    let accessor = self.mapping.get(selector);

                    // the lack of an accessor associated with a particular
                    // selector is invalid, however the value accessed by the
                    // accessor might not exist, in which case the placeholder
                    // "NA" is used
                    match accessor {
                        Some(f) => {
                            let out = f(&obj).or(Some("NA".to_owned())).unwrap();
                            output.push_str(&out);
                        },
                        None => return Err(anyhow!("unknown selector \"{}\"", &selector)),
                    }
                },
            }
        }
        Ok(output)
    }
}
