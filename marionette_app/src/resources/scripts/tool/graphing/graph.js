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

    randomize() {
        if (this.root == null) this.root = new GraphNode(this);

        currentQueue = [this.root];
        nextQueue = [];
        nodeCount = Math.floor(Math.random() * 20);

        while (currentQueue.length > 0 && nodeCount > 0) {
            current = currentQueue.pop();
            let numEdges = Math.floor(Math.random() * 3);
            for (let i = 0; i < numEdges; i++) {
                let newNode = new GraphNode();
                current.edges.push(newNode);
                nextQueue.push(newNode);
                nodeCount--;
            }
        }
    }

    layerOrdering() {
        let layers = [];
        let nextQueue = [this.root];
        let currentQueue = [];

        while (nextQueue.length > 0) {
            currentQueue = nextQueue;
            nextQueue = [];

            let layer = [];
            for (let i = 0; i < currentQueue.length; i++) {
                let node = currentQueue[i];
                layer.push(node);
                nextQueue = nextQueue.concat(node.edges);
            }

            layers.push(layer);
        }
    }
}