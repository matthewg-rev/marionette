class Graph {
    constructor() {
        this.nodes = [];
        this.renderer = new BoxRenderer();
        this.root = new GraphNode(this);

        //this.root.edges.push(new GraphNode());
        //this.root.edges.push(new GraphNode());
        //this.root.edges[0].push(new GraphNode());
        //this.root.renderer = new BoxRenderer(this.root);
    }
}