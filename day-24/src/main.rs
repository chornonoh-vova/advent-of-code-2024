use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GateOp {
    And,
    Or,
    Xor,
}

impl std::str::FromStr for GateOp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err("Unknown gate operation".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    op: GateOp,
    out: String,
}

impl Gate {
    fn new(def: &str) -> Self {
        let (left, out) = def
            .split_once(" -> ")
            .expect("Definition to contain delimiter");

        let mut iter = left.split_whitespace();
        let in1 = iter.next().expect("Definition to contain input 1");
        let op = iter.next().expect("Definition to contain operation");
        let in2 = iter.next().expect("Definition to contain input 2");

        Self {
            in1: in1.to_string(),
            in2: in2.to_string(),
            op: GateOp::from_str(op).expect("Definition to have valid operation"),
            out: out.to_string(),
        }
    }

    fn operate(&self, in1: bool, in2: bool) -> bool {
        match self.op {
            GateOp::And => in1 & in2,
            GateOp::Or => in1 | in2,
            GateOp::Xor => in1 ^ in2,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();

    let (w_str, g_str) = input
        .split_once("\n\n")
        .expect("Input to contain info for wires and gates");

    for wire_def in w_str.lines() {
        let (wire, val) = wire_def
            .split_once(": ")
            .expect("Wire definition to have delimiter");

        let w = wire.to_string();
        let v = match val {
            "1" => true,
            "0" => false,
            _ => unreachable!(),
        };

        wires.insert(w, v);
    }

    for gate_def in g_str.lines() {
        gates.push(Gate::new(gate_def));
    }

    (wires, gates)
}

fn simulate(wires: &mut HashMap<String, bool>, gates: &[Gate]) {
    let mut queue = VecDeque::from(gates.to_vec());

    while let Some(gate) = queue.pop_back() {
        if !wires.contains_key(&gate.in1) || !wires.contains_key(&gate.in2) {
            queue.push_front(gate);
            continue;
        }

        let in1 = wires[&gate.in1];
        let in2 = wires[&gate.in2];

        let out = gate.operate(in1, in2);
        wires.insert(gate.out, out);
    }
}

fn get_output(input: &str) -> usize {
    let (mut wires, gates) = parse_input(input);

    simulate(&mut wires, &gates);

    let mut out_wires: Vec<(String, bool)> = wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k.clone(), v))
        .collect();
    out_wires.sort_by_key(|(k, _)| k.clone());

    println!("{:?}", out_wires);

    let mut out = 0;
    let mut mult = 1;

    for (_, val) in out_wires {
        if val {
            out += mult;
        }
        mult *= 2;
    }

    out
}

fn get_wires_to_swap(input: &str) -> String {
    let (_, gates) = parse_input(input);
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    for gate in &gates {
        edges
            .entry(gate.in1.clone())
            .or_default()
            .push(gate.out.clone());
        edges
            .entry(gate.in2.clone())
            .or_default()
            .push(gate.out.clone());
    }

    let mut broken_nodes = HashSet::new();

    for g in &gates {
        // z nodes must be XOR (except for the last one, z45)
        if g.out.starts_with("z") && g.out != "z45" && g.op != GateOp::Xor {
            broken_nodes.insert(g.out.clone());
        }
        // z nodes must not be inputs of other nodes
        if g.in1.starts_with("z") {
            broken_nodes.insert(g.in1.clone());
        }
        if g.in2.starts_with("z") {
            broken_nodes.insert(g.in2.clone());
        }

        // inputs of XOR nodes (except for z nodes) must be x and y nodes
        if g.op == GateOp::Xor
            && !g.out.starts_with("z")
            && !((g.in1.starts_with("x") && g.in2.starts_with("y"))
                || (g.in1.starts_with("y") && g.in2.starts_with("x")))
        {
            broken_nodes.insert(g.out.clone());
        }

        // XOR nodes (except z nodes) must always be input of exactly two
        // other nodes
        if g.op == GateOp::Xor && !g.out.starts_with("z") && edges[&g.out].len() != 2 {
            broken_nodes.insert(g.out.clone());
        }

        // AND nodes must always be input of exactly one other node (except
        // the very first one wired to x00 and y00)
        if g.op == GateOp::And
            && !g.out.starts_with("z")
            && edges[&g.out].len() != 1
            && !((g.in1 == "x00" && g.in2 == "y00") || (g.in1 == "y00" && g.in2 == "x00"))
        {
            broken_nodes.insert(g.out.clone());
        }
    }

    let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
    broken_nodes.sort();
    broken_nodes.join(",")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].as_str();
    let input = std::fs::read_to_string(filename).expect("Failed to read file");

    println!("simulation output: {}", get_output(input.as_str()));
    println!("wrong wires: {}", get_wires_to_swap(input.as_str()));
}
