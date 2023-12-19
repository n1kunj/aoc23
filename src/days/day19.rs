use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn new(s: &str) -> Category {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            other => panic!("{other}"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Send { dst: String },
    Accept,
    Reject,
}

impl Action {
    fn new(s: &str) -> Action {
        match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            other => Action::Send {
                dst: other.to_owned(),
            },
        }
    }
}

#[derive(Debug)]
enum Rule {
    IfGreaterThan {
        cat: Category,
        val: usize,
        act: Action,
    },
    IfLessThan {
        cat: Category,
        val: usize,
        act: Action,
    },
    Unconditionally {
        act: Action,
    },
}

static RULE_GT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)>(\d+):(\w+)").unwrap());
static RULE_LE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)<(\d+):(\w+)").unwrap());

impl Rule {
    fn new(s: &str) -> Rule {
        if let Some(c) = RULE_GT_RE.captures(s) {
            Rule::IfGreaterThan {
                cat: Category::new(c.get(1).unwrap().as_str()),
                val: c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                act: Action::new(c.get(3).unwrap().as_str()),
            }
        } else if let Some(c) = RULE_LE_RE.captures(s) {
            Rule::IfLessThan {
                cat: Category::new(c.get(1).unwrap().as_str()),
                val: c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                act: Action::new(c.get(3).unwrap().as_str()),
            }
        } else {
            Rule::Unconditionally {
                act: Action::new(s),
            }
        }
    }

    fn apply(&self, part: &Part) -> Option<&Action> {
        match self {
            Rule::IfGreaterThan { cat, val, act } => {
                if part.get(*cat) > *val {
                    Some(act)
                } else {
                    None
                }
            }
            Rule::IfLessThan { cat, val, act } => {
                if part.get(*cat) < *val {
                    Some(act)
                } else {
                    None
                }
            }
            Rule::Unconditionally { act } => Some(act),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

static WORKFLOW_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(.+)\}").unwrap());

impl Workflow {
    fn new(s: &str) -> Workflow {
        let c = WORKFLOW_RE.captures(s).unwrap();
        let name = c.get(1).unwrap().as_str();
        let mut rules = Vec::<Rule>::new();
        for rule in c.get(2).unwrap().as_str().split(",") {
            rules.push(Rule::new(rule));
        }
        Workflow {
            name: name.to_owned(),
            rules,
        }
    }
}

static PART_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap());

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(s: &str) -> Part {
        let c = PART_RE.captures(s).unwrap();
        let parse = |i: usize| -> usize { c.get(i).unwrap().as_str().parse::<usize>().unwrap() };
        Part {
            x: parse(1),
            m: parse(2),
            a: parse(3),
            s: parse(4),
        }
    }

    fn get(&self, c: Category) -> usize {
        match c {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn main(input: &str) -> (String, String) {
    let mut workflows = Vec::<Workflow>::new();
    let mut parts = Vec::<Part>::new();

    let mut is_part_line = false;
    for l in input.lines() {
        if l.is_empty() {
            is_part_line = true;
            continue;
        }

        if is_part_line {
            parts.push(Part::new(l));
        } else {
            workflows.push(Workflow::new(l));
        }
    }

    let name_to_wf = workflows
        .iter()
        .map(|wf| (wf.name.as_str(), wf))
        .collect::<HashMap<_, _>>();

    let in_wf = name_to_wf["in"];

    let mut accepted_sum = 0usize;
    for part in parts.iter() {
        let mut wf = in_wf;
        let mut rule_idx = 0usize;
        loop {
            let rule = &wf.rules[rule_idx];
            let act = rule.apply(part);
            match act {
                Some(act) => match act {
                    Action::Send { dst } => {
                        wf = &name_to_wf[dst.as_str()];
                        rule_idx = 0;
                    }
                    Action::Accept => {
                        accepted_sum += part.sum();
                        break;
                    }
                    Action::Reject => {
                        break;
                    }
                },
                None => rule_idx += 1,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Ranges {
        x: (usize, usize),
        m: (usize, usize),
        a: (usize, usize),
        s: (usize, usize),
    }

    impl Ranges {
        fn split(&self, c: Category, v: usize) -> (Ranges, Ranges) {
            let mut r1 = *self;
            let mut r2 = *self;
            let (a, b, c) = match c {
                Category::X => (self.x, &mut r1.x, &mut r2.x),
                Category::M => (self.m, &mut r1.m, &mut r2.m),
                Category::A => (self.a, &mut r1.a, &mut r2.a),
                Category::S => (self.s, &mut r1.s, &mut r2.s),
            };

            let clamped = v.clamp(a.0, a.1);
            b.1 = clamped;
            c.0 = clamped;

            (r1, r2)
        }

        fn combinations(&self) -> usize {
            let combos = |v: (usize, usize)| -> usize { v.1 - v.0 };
            combos(self.x) * combos(self.m) * combos(self.a) * combos(self.s)
        }
    }

    struct RuleHead<'a> {
        wf: &'a Workflow,
        rule_idx: usize,
        ranges: Ranges,
    }

    let mut heads = Vec::<RuleHead>::new();

    heads.push(RuleHead {
        wf: in_wf,
        rule_idx: 0,
        ranges: Ranges {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        },
    });

    let mut acc_ranges = Vec::<Ranges>::new();
    let mut rej_ranges = Vec::<Ranges>::new();

    let mut next_heads = Vec::<RuleHead>::new();
    while !heads.is_empty() {
        for h in heads.drain(..) {
            let rule = &h.wf.rules[h.rule_idx];

            let mut handle_action = |act: Option<&Action>, r: Ranges| match act {
                Some(act) => match act {
                    Action::Send { dst } => next_heads.push(RuleHead {
                        wf: &name_to_wf[dst.as_str()],
                        rule_idx: 0,
                        ranges: r,
                    }),
                    Action::Accept => {
                        acc_ranges.push(r);
                    }
                    Action::Reject => {
                        rej_ranges.push(r);
                    }
                },
                None => next_heads.push(RuleHead {
                    wf: h.wf,
                    rule_idx: h.rule_idx + 1,
                    ranges: r,
                }),
            };

            match rule {
                Rule::IfGreaterThan { cat, val, act } => {
                    let (no, yes) = h.ranges.split(*cat, *val + 1);
                    handle_action(Some(act), yes);
                    handle_action(None, no);
                }
                Rule::IfLessThan { cat, val, act } => {
                    let (yes, no) = h.ranges.split(*cat, *val);
                    handle_action(Some(act), yes);
                    handle_action(None, no);
                }
                Rule::Unconditionally { act } => handle_action(Some(act), h.ranges),
            }
        }
        heads.extend(next_heads.drain(..));
    }

    let combinations = acc_ranges.iter().map(|r| r.combinations()).sum::<usize>();

    (format!("{accepted_sum}"), format!("{combinations}"))
}
