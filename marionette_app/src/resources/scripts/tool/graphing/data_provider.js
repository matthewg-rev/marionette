class DataProvider {
    constructor() {}
    provide(node) {}
}

class DebugDataProvider extends DataProvider {
    constructor() {
        super();
        this.instructions = ["LOADK", "FORPREP", "FORLOOP", "MOVE", "GETUPVAL", "SETUPVAL", "GETTABUP", "GETTABLE", "SETTABUP", "SETTABLE", "NEWTABLE", "SELF", "ADD", "SUB", "MUL", "DIV", "MOD", "POW", "UNM", "NOT", "LEN", "CONCAT", "JMP", "EQ", "LT", "LE", "TEST", "TESTSET", "CALL", "TAILCALL", "RETURN", "FORCALL", "TFORLOOP", "SETLIST", "CLOSURE", "VARARG"];
        this.fakeAddress = 0;
    }

    provide(node) {
        // Clear the existing content of the node
        node.content = [];

        // Determine the number of lines to generate, between 1 and 10
        const lineAmt = Math.floor(Math.random() * 10) + 1;

        // Iterate to create each line
        for (let i = 0; i < lineAmt; i++) {
            // Create a new line with a specific font and color
            const line = new Line("Consolas", "#9b9b9b");

            // Increment the fake address
            this.fakeAddress++;
            const addressColor = "#3FAAB5";
            // add fakeAddress to the line as hex
            line.add(`${this.fakeAddress.toString(16).padStart(4, '0')}\t`, addressColor);

            // Select a random instruction from the predefined list
            const word = this.instructions[Math.floor(Math.random() * this.instructions.length)];

            // Assign a specific color based on the instruction type
            const instructionColor = (word === "LOADK" || word === "FORPREP") ? "#AD9764" : "#B686C1";
            line.add(`${word}\t`, instructionColor);

            // Define a consistent color for random numbers
            const randomColor = "#3FAAB5";

            // Add three segments of random numbers to the line, sometimes adding a fourth
            line.add(`${Math.floor(Math.random() * 100)}\t`, randomColor);
            line.add(`${Math.floor(Math.random() * 100)}\t`, randomColor);
            if (Math.random() > 0.5) {
                line.add(`${Math.floor(Math.random() * 100)}\t`, randomColor);
            }

            // Append the constructed line to the node's content
            node.content.push(line);
        }
    }
}