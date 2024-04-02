class GraphRenderer {
    static Config() { 
        return RendererConfig.Configs[GraphRenderer];
    }
    constructor() {}

    preprocess(ctx, data) {}
    render(ctx, widget) {}
}

class VertexRenderer {
    static Config() { 
        return RendererConfig.Configs[VertexRenderer]; 
    }
    constructor() {}

    preprocess(ctx, data) {}
    metrics(ctx, vertex) {}
    render(ctx, widget) {}
}

class EdgeRenderer {
    static Config() {
        return RendererConfig.Configs[EdgeRenderer];
    }
    constructor() {}

    preprocess(ctx, data) {}
    render(ctx, widget) {}
}

class ErrorRenderer {
    static Config() {
        return RendererConfig.Configs[ErrorRenderer];
    }
    constructor() {}

    preprocess(ctx, data) {}
    render(ctx, widget) {}
}

class RendererConfig {
    static Configs = {
        [GraphRenderer]: {},
        [VertexRenderer]: {
            FONTS: {
                FONT_CONTENT: "Consolas"
            },
            COLORS: {
                COLOR_BORDER: "#9b9b9b",
                COLOR_BACKGROUND: "#0f0f0f",
                COLOR_SHADOW: "#080808"
            },
            SIZES: {
                SIZE_TEXT: 16,
                SIZE_BORDER: 1,
                SIZE_SHADOW: 1
            },
            OFFSETS: {
                OFFSET_SHADOW: 4
            },
            PADDING: {
                PADDING_BOX_HORIZONTAL: 10,
                PADDING_BOX_VERTICAL: 20,
                PADDING_LINE: 5
            },
            RENDERING: {
                VERTEX_CENTERING: true
            }
        },
        [EdgeRenderer]: {
            COLORS: {
                COLOR_DIRECT: "#9b9b9b",
                COLOR_TRUE: "#7fff7f",
                COLOR_FALSE: "#ff7f7f"
            },
            PADDING: {
                PADDING_BETWEEN_EDGES: 0.05, // 5% of width
            },
            SIZES: {
                SIZE_LINE_START: 2
            }
        },
        [ErrorRenderer]: {}
    };
}