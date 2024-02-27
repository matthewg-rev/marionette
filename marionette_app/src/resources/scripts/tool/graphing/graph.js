class Graph {
    constructor() {
        this.root = new GraphNode();
        this.root.data = [
            new Line("Consolas", "#9b9b9b")
                .add("LOADK\t", "#AD9764")
                .add("0\t", "#3FAAB5")
                .add("0\t", "#3FAAB5"),
            new Line("Consolas", "#9b9b9b")
                .add("LOADK\t", "#AD9764")
                .add("1\t", "#3FAAB5")
                .add("1\t", "#3FAAB5"),
            new Line("Consolas", "#9b9b9b")
                .add("LOADK\t", "#AD9764")
                .add("2\t", "#3FAAB5")
                .add("2\t", "#3FAAB5"),
            new Line("Consolas", "#9b9b9b")
                .add("FORPREP\t", "#AD9764")
                .add("0\t", "#3FAAB5")
                .add("13\t", "#3FAAB5"),
        ];
        this.root.renderer = new BoxRenderer(this.root);
    }
}