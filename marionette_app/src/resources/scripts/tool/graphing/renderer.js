class GraphRenderer {
    constructor() {}

    render(node, widget, ctx) {}
}

class VertexRenderer {
    constructor() {}

    longestText(ctx, text) {}
    vertexSize(vertex, ctx) {}
    textHeight(vertex, ctx) {}

    renderContents(vertex, ctx, vertexX, vertexY) {}
    render(vertex, widget, ctx, vertexX, vertexY) {}
}

class EdgeRenderer {
    constructor() {}

    render(source, target, widget, ctx) {}
}

class ErrorRenderer {
    constructor() {}

    render(message, widget, ctx) {}
}