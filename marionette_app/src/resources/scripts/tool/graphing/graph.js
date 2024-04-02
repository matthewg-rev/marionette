class Graph {
    constructor() {
        this.nodes = [];
        this.edges = [];
        this.root = new GraphVertex(this);
    }

    static random() {
        let graph = new Graph();
        let nodes = Math.floor(Math.random() * 10) + 1;

        let current = graph.root;
        for (let i = 0; i < nodes; i++) {
            let node = new GraphVertex(graph);
            graph.edges.push(new GraphEdge(current, node));
            if (Math.random() > 0.5) {
                current = node;
            }
        }

        return graph;
    }

    updateIdentifiers() {
        this.nodes.forEach((node, index) => {
            node.id = index;
        });
    }

    verifyIntegrity() {
        let valid = true;
        this.nodes.forEach((node) => {
            if (node.id === -1) {
                valid = false;
            }
        });
        return valid;
    }
}