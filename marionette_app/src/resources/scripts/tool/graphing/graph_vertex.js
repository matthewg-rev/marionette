class GraphVertex {
    static provider = new DebugDataProvider();

    constructor(graph) {
        // (* content that will be rendered in the vertex                                  *)
        this.content = [];
        GraphVertex.provider.provide(this);

        // (* unique identifier for the vertex                                             *)
        this.id = -1;

        graph.nodes.push(this);
    }
}