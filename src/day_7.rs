use std::collections::HashMap;
use std::collections::HashSet;

// this is much easier than the first part
// because we only see each parent once
// parsing the file.
pub fn part_2() {
    let mut line_var = String::from("");
    let mut graph: HashMap<String, HoldNode> = HashMap::new();
    with_read_lines!(
        "./data/day_7.txt",
        line_var,
        {
            let ruleple = parse_rule(line_var);
            let parent = ruleple.parent;
            match graph.get(&parent) {
                Some(hold_node) => {
                    println!("I didn't think I'd get back to {}", parent);
                }
                _ => {
                    let mut children: HashMap<String, usize> = HashMap::new();
                    for child in &ruleple.children {
                        children.insert(child.1.to_string(), child.0.clone());
                    }
                    graph.insert(
                        String::from(&parent),
                        HoldNode {
                            id: String::from(&parent),
                            children: children,
                        },
                    );
                }
            }
        },
        {
            let top_bag = String::from("shiny gold");
            let count = total_bags(top_bag, &graph);
            println!("{} bags!", count);
        }
    );
}

fn total_bags(bag: String, graph: &HashMap<String, HoldNode>) -> usize {
    let mut total = 0;
    if let Some(next_node) = graph.get(&bag) {
        if next_node.children.is_empty() {
            total = 0;
        } else {
            for (child, required) in &next_node.children {
                total += (required * total_bags(child.to_string(), graph));
                total += required;
            }
        }
    } else {
        println!("{} not found, that's bad news", bag);
    }

    total
}

pub fn part_1() {
    let mut line_var = String::from("");
    let mut graph: HashMap<String, BagNode> = HashMap::new();
    with_read_lines!(
        "./data/day_7.txt",
        line_var,
        {
            let ruleple = parse_rule(line_var);
            let parent = ruleple.parent;
            for child in &ruleple.children {
                match graph.get(&child.1) {
                    Some(bag_node) => {
                        //don't love how costly this will be...
                        let mut updated_required: Vec<usize> = Vec::new();
                        for b in &bag_node.required {
                            updated_required.push(b.clone());
                        }
                        updated_required.push(child.0);

                        let mut updated_parent: Vec<String> = Vec::new();
                        for p in &bag_node.parent {
                            updated_parent.push(String::from(p));
                        }
                        updated_parent.push(String::from(&parent));
                        graph.remove(&child.1);
                        graph.insert(
                            String::from(&child.1),
                            BagNode {
                                id: String::from(&child.1),
                                required: updated_required,
                                parent: updated_parent,
                            },
                        );
                    }

                    _ => {
                        graph.insert(
                            String::from(&child.1),
                            BagNode {
                                id: String::from(&child.1),
                                required: vec![child.0],
                                parent: vec![String::from(&parent)],
                            },
                        );
                    }
                }
            }
        },
        {
            let mut visited = HashSet::<String>::new();
            let mut count: usize = 0;
            let deepest_bag = String::from("shiny gold");
            let mut queue: Vec<String> = Vec::new();
            queue.push(deepest_bag);
            while queue.len() > 0 {
                if let Some(next_node) = graph.get(&queue[0]) {
                    for parent in &next_node.parent {
                        if !visited.contains(parent) {
                            visited.insert(parent.to_string());
                            queue.push(parent.to_string());
                        }
                    }
                } else {
                    // println!("{} not found", queue[0]);
                }
                queue.remove(0);
            }
            println!("{} bags in bags!", visited.len());
        }
    );
}

fn parse_rule(raw_rule: String) -> Rule {
    let parts = raw_rule.split(" bags contain ").collect::<Vec<&str>>();
    let parent = parts[0];
    let mut children: Vec<(usize, String)> = Vec::new();
    for child in parts[1].split(", ").collect::<Vec<&str>>() {
        let mut child_parts = child.split(" ").collect::<Vec<&str>>();
        if let Ok(required) = child_parts[0].parse::<usize>() {
            let child_id = &child_parts[1..3].join(" ");
            &children.push((required, String::from(child_id)));
        }
    }
    Rule {
        parent: String::from(parent),
        children: children,
    }
}

struct Rule {
    parent: String,
    children: Vec<(usize, String)>,
}

struct BagNode {
    id: String,
    required: Vec<usize>,
    parent: Vec<String>, //torn about this representation but ids should work
}
//poor naming with this, but this is for parent -> children relationships
struct HoldNode {
    id: String,
    children: HashMap<String, usize>,
}
