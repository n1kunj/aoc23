use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    None,
    Conjunction,
    FlipFlop,
}

#[derive(Debug)]
struct Module {
    t: ModuleType,
    n: String,
    d: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum State {
    High,
    Low,
}

impl State {
    fn flip(&self) -> State {
        match self {
            State::High => State::Low,
            State::Low => State::High,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleState {
    None,
    Conjunction(HashMap<usize, State>),
    FlipFlop(State),
}

impl ModuleState {
    fn update(&mut self, src_idx: usize, in_state: State) -> Option<State> {
        match self {
            ModuleState::None => Some(in_state),
            ModuleState::Conjunction(s) => {
                assert!(s.contains_key(&src_idx));
                s.insert(src_idx, in_state);
                // s[&src_idx] = in_state;
                if s.iter().all(|(_, v)| matches!(v, State::High)) {
                    Some(State::Low)
                } else {
                    Some(State::High)
                }
            }
            ModuleState::FlipFlop(s) => match in_state {
                State::High => None,
                State::Low => {
                    *s = s.flip();
                    Some(*s)
                }
            },
        }
    }
}

struct LiveModule<'a> {
    #[allow(dead_code)]
    module: &'a Module,
    state: ModuleState,
    dst_idxs: Vec<usize>,
}

pub fn main(input: &str) -> (String, String) {
    let mut modules = Vec::<Module>::new();

    modules.push(Module {
        t: ModuleType::None,
        n: "button".to_owned(),
        d: vec!["broadcaster".to_owned()],
    });

    for l in input.lines() {
        let mut split = l.split(" -> ");
        let src = split.next().unwrap();
        let dst = split.next().unwrap();
        let dsts: Vec<String> = dst.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>();

        let module = if src.starts_with("%") {
            Module {
                t: ModuleType::FlipFlop,
                n: src[1..].to_owned(),
                d: dsts,
            }
        } else if src.starts_with("&") {
            Module {
                t: ModuleType::Conjunction,
                n: src[1..].to_owned(),
                d: dsts,
            }
        } else {
            Module {
                t: ModuleType::None,
                n: src.to_owned(),
                d: dsts,
            }
        };

        modules.push(module);
    }

    let name_to_idx = modules
        .iter()
        .enumerate()
        .map(|m| (m.1.n.as_str(), m.0))
        .collect::<HashMap<_, _>>();

    let mut output_modules = HashSet::<String>::new();

    for module in modules.iter() {
        for dst in module.d.iter() {
            if !name_to_idx.contains_key(dst.as_str()) {
                output_modules.insert(dst.clone());
            }
        }
    }

    modules.extend(output_modules.drain().map(|n| Module {
        t: ModuleType::None,
        n,
        d: vec![],
    }));

    let name_to_idx = modules
        .iter()
        .enumerate()
        .map(|m| (m.1.n.as_str(), m.0))
        .collect::<HashMap<_, _>>();

    let mut live_modules = Vec::<LiveModule>::new();

    fn input_idxes(name: &str, modules: &Vec<Module>) -> Vec<usize> {
        let mut idxes = Vec::<usize>::new();
        for (n_module_idx, n_module) in modules.iter().enumerate() {
            for dst in n_module.d.iter() {
                if dst.as_str() == name {
                    idxes.push(n_module_idx);
                }
            }
        }
        idxes
    }

    for module in modules.iter() {
        let state = match module.t {
            ModuleType::None => ModuleState::None,
            ModuleType::Conjunction => {
                let hm = input_idxes(&module.n, &modules)
                    .iter()
                    .map(|n| (*n, State::Low))
                    .collect::<HashMap<_, _>>();
                ModuleState::Conjunction(hm)
            }
            ModuleType::FlipFlop => ModuleState::FlipFlop(State::Low),
        };
        let dst_idxs = module
            .d
            .iter()
            .map(|s| name_to_idx[s.as_str()])
            .collect::<Vec<_>>();
        live_modules.push(LiveModule {
            module,
            state,
            dst_idxs,
        });
    }

    let has_rx = name_to_idx.contains_key("rx");

    let needs_low_iteration_record = if has_rx {
        let n0 = input_idxes("rx", &modules);

        let get_next_input_idxes = |idxs: &Vec<usize>| -> Vec<usize> {
            let mut next_input_idxes = Vec::<usize>::new();
            for idx in idxs.iter() {
                next_input_idxes.extend(input_idxes(&modules[*idx].n, &modules));
            }
            next_input_idxes
        };

        let n1 = get_next_input_idxes(&n0);

        n1
    } else {
        vec![]
    };

    let button_idx = name_to_idx["button"];
    let broadcaster_idx = name_to_idx["broadcaster"];

    let mut pulses = Vec::<(usize, usize, State, &str, &str)>::new();
    let mut next_pulses = Vec::<(usize, usize, State, &str, &str)>::new();

    let mut low_iterations = modules
        .iter()
        .map(|_| Vec::<usize>::new())
        .collect::<Vec<_>>();

    let mut low_pulse_count = 0usize;
    let mut high_pulse_count = 0usize;
    let mut button_push_idx = 0usize;
    loop {
        pulses.push((
            button_idx,
            broadcaster_idx,
            State::Low,
            "button",
            "broadcaster",
        ));
        while !pulses.is_empty() {
            for (src_idx, dst_idx, in_state, _src_name, _dst_name) in pulses.drain(..) {
                if button_push_idx < 1000 {
                    match in_state {
                        State::High => high_pulse_count += 1,
                        State::Low => low_pulse_count += 1,
                    }
                }

                if matches!(in_state, State::Low) {
                    low_iterations[dst_idx].push(button_push_idx);
                }

                let dst_name = modules[dst_idx].n.as_str();

                let out_state = live_modules[dst_idx].state.update(src_idx, in_state);

                match out_state {
                    Some(out_state) => {
                        for next_dst_idx in live_modules[dst_idx].dst_idxs.iter() {
                            let next_dst_name = modules[*next_dst_idx].n.as_str();
                            next_pulses.push((
                                dst_idx,
                                *next_dst_idx,
                                out_state,
                                dst_name,
                                next_dst_name,
                            ));
                        }
                    }
                    None => (),
                }
            }
            pulses.extend(next_pulses.drain(..));
        }
        button_push_idx += 1;

        if button_push_idx >= 1000
            && needs_low_iteration_record
                .iter()
                .all(|n| low_iterations[*n].len() >= 3)
        {
            break;
        }
    }
    let product = low_pulse_count * high_pulse_count;

    let rx_low_pulse_min = if has_rx {
        let deltas = needs_low_iteration_record
            .iter()
            .map(|n| low_iterations[*n][2] - low_iterations[*n][1])
            .collect::<Vec<_>>();

        let mut m = 1usize;
        for d in deltas {
            m *= d;
        }
        m
    } else {
        usize::MAX
    };

    (format!("{product}"), format!("{rx_low_pulse_min}"))
}
