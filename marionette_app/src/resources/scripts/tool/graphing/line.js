class Line {
    constructor(font, color, text) {
        this.font = font;
        this.color = color;
        this.content = [];
        if (text) {
            this.add(text);
        }
    }

    add(text, color) {
        if (text instanceof Text) {
            this.content.push(text);
        } else if (typeof text === "string") {
            if (color) {
                this.content.push(new Text(text, color));
            } else {
                this.content.push(new Text(text, this.color));
            }
        }
        return this;
    }

    raw() {
        return this.content.map((t) => t.content).join();
    }
}