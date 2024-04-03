class GraphVertex {
    static provider = new DebugDataProvider();

    constructor(graph) {
        // (* content that will be rendered in the vertex                                  *)
        this.content = [];
        GraphVertex.provider.provide(this);

        // (* unique identifier for the vertex                                             *)
        this.id = -1;

        // (* whether or not the user has selected the vertex                              *)
        this.selected = false;

        graph.nodes.push(this);
    }

    static fromJSON(graph, json) {
        let vertex = new GraphVertex(graph);
        vertex.id = json.id;
        return vertex;
    }
}