class Graph {
    constructor() {
        this.nodes = [];
        this.edges = [];
        this.root = new GraphVertex(this);
    }

    static fromJSON(json) {
        let graph = new Graph();
        graph.nodes = json.nodes.map((node) => GraphVertex.fromJSON(graph, node));
        graph.edges = json.edges.map((edge) => GraphEdge.fromJSON(graph, edge));
        graph.root = graph.nodes[0];
        return graph;
    }

    static testJSON() {
        return {
            "nodes": [
                { "id": 0 },
                { "id": 1 },
                { "id": 2 },
                { "id": 3 },
                { "id": 4 },
                { "id": 5 },
                { "id": 6 },
                { "id": 7 },
                { "id": 8 },
                { "id": 9 },
                { "id": 10 },
                { "id": 11 },
                { "id": 12 },
                { "id": 13 },
                { "id": 14 },
                { "id": 15 },
                { "id": 16 },
                { "id": 17 },
                { "id": 18 },
                { "id": 19 },
                { "id": 20 },
            ],
            "edges": [
                { "source": 0, "target": 1 },
                { "source": 0, "target": 5 },
                { "source": 1, "target": 2 },
                { "source": 1, "target": 3 },
                { "source": 2, "target": 10 },
                { "source": 2, "target": 7 },
                { "source": 3, "target": 4 },
                { "source": 3, "target": 9 },
                { "source": 4, "target": 11 },
                { "source": 5, "target": 6 },
                { "source": 5, "target": 8 },
                { "source": 6, "target": 12 },
                { "source": 6, "target": 9 },
                { "source": 7, "target": 8 },
                { "source": 7, "target": 10 },
                { "source": 8, "target": 9 },
                { "source": 8, "target": 13 },
                { "source": 9, "target": 10 },
                { "source": 10, "target": 12 },
                { "source": 10, "target": 14 },
                { "source": 11, "target": 13 },
                { "source": 12, "target": 15 },
                { "source": 14, "target": 16 },
                { "source": 15, "target": 18 },
                { "source": 16, "target": 17 },
                { "source": 17, "target": 19 },
                { "source": 18, "target": 19 },
                { "source": 19, "target": 20 }
            ]
        };
    }

    static random() {
        let graph = new Graph();
        let nodes = Math.floor(Math.random() * 10) + 1;

        let current = graph.root;
        for (let i = 0; i < nodes; i++) {
            let node = new GraphVertex(graph);
            graph.edges.push(new GraphEdge(current, node));
            let edges = graph.edges.filter((edge) => edge.source === current);
            if (Math.random() > 0.5 || edges.length == 2) {
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