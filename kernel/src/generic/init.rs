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
        assert_eq!(self.done.load(Ordering::Relaxed), false);
        assert_eq!(self.unsatisfied_deps.load(Ordering::Relaxed), 0);
        (self.action)();
        self.done.store(true, Ordering::Relaxed);
        info!("Reached stage {:?}", self.display_name);
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

pub fn run() {
    let mut ctors_start = &raw const LD_INIT_CTORS_START as *const fn();
    let ctors_end = &raw const LD_INIT_CTORS_END as *const fn();
    while ctors_start < ctors_end {
        unsafe {
            (*ctors_start)();
            ctors_start = ctors_start.add(1);
        }
    }

    let start = &raw const LD_INIT_START as *const Node;
    let end = &raw const LD_INIT_END as *const Node;

    let nodes = unsafe { core::slice::from_raw_parts(start, end.offset_from_unsigned(start)) };
    let mut pending = LinkedList::new(PendingLinkAdapter::NEW);

    for node in nodes {
        if node.unsatisfied_deps.load(Ordering::Relaxed) == 0 {
            pending.push_back(node);
        }
    }

    while let Some(node) = pending.pop_front() {
        node.run();

        for edge in node.out_edges.lock().iter() {
            if edge.to.unsatisfied_deps.fetch_sub(1, Ordering::Relaxed) == 1 {
                pending.push_back(edge.to);
            }
        }
    }

    for node in nodes {
        assert!(
            node.done.load(Ordering::Relaxed),
            "The dependencies for node {} could not be resolved!",
            node.display_name
        );
    }

    info!("All stages are complete!");

    if BootInfo::get()
        .command_line
        .get_bool("initgraph")
        .unwrap_or(false)
    {
        let mut graph = String::new();
        graph += "subgraph{";
        for node in nodes {
            graph += &format!("n{:p} [label={:?}];", node, node.display_name);
            for edge in node.in_edges.lock().iter() {
                graph += &format!("n{:p} -> n{:p};", edge.from, edge.to);
            }
        }
        graph += "}";

        log!("{}", graph);
    }
}

unsafe extern "C" {
    unsafe static LD_INIT_CTORS_START: u8;
    unsafe static LD_INIT_CTORS_END: u8;
    unsafe static LD_INIT_START: u8;
    unsafe static LD_INIT_END: u8;
}
