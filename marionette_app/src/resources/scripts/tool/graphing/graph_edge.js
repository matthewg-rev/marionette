class GraphEdge {
    constructor(source, target) {
        this.source = source;
        this.target = target;
    }

    static fromJSON(graph, json) {
        return new GraphEdge(graph.nodes[json.source], graph.nodes[json.target]);
    }

    getSource() {
        return this.source;
    }

    getTarget() {
        return this.target;
    }
}