//! Initialization sequence and task management.

#![allow(unused)]
use crate::generic::{boot::BootInfo, memory::virt, util::mutex::Mutex};
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use intrusive_collections::{DefaultLinkOps, LinkedList, LinkedListLink, intrusive_adapter};

pub struct Node {
    display_name: &'static str,
    done: AtomicBool,
    unsatisfied_deps: AtomicUsize,

    in_edges: Mutex<LinkedList<InLinkAdapter>>,
    out_edges: Mutex<LinkedList<OutLinkAdapter>>,
    pending_link: LinkedListLink,

    action: fn(),
}

unsafe impl Sync for Node {}

intrusive_adapter!(InLinkAdapter = &'static Edge: Edge { in_link: LinkedListLink });
intrusive_adapter!(OutLinkAdapter = &'static Edge: Edge { out_link: LinkedListLink });
intrusive_adapter!(PendingLinkAdapter = &'static Node: Node { pending_link: LinkedListLink });

impl Node {
    pub const fn new(display_name: &'static str, action: fn()) -> Self {
        Self {
            done: AtomicBool::new(false),
            display_name,
            action,
            unsatisfied_deps: AtomicUsize::new(0),
            in_edges: Mutex::new(LinkedList::new(InLinkAdapter::NEW)),
            out_edges: Mutex::new(LinkedList::new(OutLinkAdapter::NEW)),
            pending_link: LinkedListLink::new(),
        }
    }

    fn run(&self) {
        assert!(!self.done.load(Ordering::Relaxed));
        assert_eq!(self.unsatisfied_deps.load(Ordering::Relaxed), 0);
        (self.action)();
        self.done.store(true, Ordering::Relaxed);
        status!("Reached stage {:?}", self.display_name);
    }
}

pub struct Edge {
    from: &'static Node,
    to: &'static Node,
    in_link: LinkedListLink,
    out_link: LinkedListLink,
}

unsafe impl Sync for Edge {}

impl Edge {
    pub const fn new(from: &'static Node, to: &'static Node) -> Self {
        Self {
            from,
            to,
            in_link: LinkedListLink::new(),
            out_link: LinkedListLink::new(),
        }
    }

    pub fn register(&'static self) {
        self.from.out_edges.lock().push_back(self);
        self.to.in_edges.lock().push_back(self);
        self.to.unsatisfied_deps.fetch_add(1, Ordering::Relaxed);
    }
}

/// Runs the global initialization sequence.
pub fn run() {
    unsafe {
        initgraph::initialize_edges();
    }

    initgraph::execute_graph(None, |node| status!("Done stage: {}", node.display_name()));

    status!("All stages are complete!");

    if BootInfo::get()
        .command_line
        .get_bool("initgraph")
        .unwrap_or(false)
    {
        let mut graph = String::new();

        graph += "digraph initgraph {\n";
        graph += "\tsubgraph {\n";

        for node in initgraph::get_all_nodes() {
            graph += &format!("\t\tn{:p} [label={:?}];\n", node, node.display_name());

            for edge in node.in_edges().iter() {
                graph += &format!("\t\t\tn{:p} -> n{:p};\n", edge.source(), edge.target());
            }
        }

        graph += "\t}\n}";

        log!("{}", graph);
    }
}
