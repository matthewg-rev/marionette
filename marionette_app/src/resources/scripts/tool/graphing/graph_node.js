class GraphNode {
    constructor(graph) {
        this.edges = [];
        this.data = null;
        graph.nodes.push(this);

        /* debug code */
        // randomly generate content
        {
            this.data = [];
            let lineAmt = Math.floor(Math.random() * 10) + 1;
            let instructions = ["LOADK", "FORPREP", "FORLOOP", "MOVE", "GETUPVAL", "SETUPVAL", "GETTABUP", "GETTABLE", "SETTABUP", "SETTABLE", "NEWTABLE", "SELF", "ADD", "SUB", "MUL", "DIV", "MOD", "POW", "UNM", "NOT", "LEN", "CONCAT", "JMP", "EQ", "LT", "LE", "TEST", "TESTSET", "CALL", "TAILCALL", "RETURN", "FORCALL", "TFORLOOP", "SETLIST", "CLOSURE", "VARARG"];
            for (let i = 0; i < lineAmt; i++) {
                let line = new Line("Consolas", "#9b9b9b");
                let word = instructions[Math.floor(Math.random() * instructions.length)];
                let color = "#B686C1";
                if (word == "LOADK" || word == "FORPREP") {
                    color = "#AD9764";
                }
                line.add(word + "\t", color);
                color = "#3FAAB5";
                line.add(Math.floor(Math.random() * 100) + "\t", color);
                line.add(Math.floor(Math.random() * 100) + "\t", color);
                if (Math.random() > 0.5) {
                    line.add(Math.floor(Math.random() * 100) + "\t", color);
                }
                this.data.push(line);
            }
        }
    }
}