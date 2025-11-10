use crate::boot::BootInfo;
use alloc::string::String;

/// Runs the global initialization sequence.
pub fn run() {
    unsafe {
        initgraph::initialize_edges();
    }

    initgraph::execute_graph(None, |node| {
        status!("Running stage \"{}\"", node.display_name())
    });

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
