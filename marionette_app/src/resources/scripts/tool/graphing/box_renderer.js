// https://www.cs.unc.edu/techreports/89-034.pdf
class BoxRenderer extends GraphRenderer {
    constructor() {
        super();
        this.vertexRenderer = new BoxVertexRenderer();
        this.edgeRenderer = new BoxEdgeRenderer();
        this.drawingData = {
            vertexData: [],
            edgeData: []
        }
    }

    render(graph, widget, ctx) {
        if (this.graph != graph) {
            this.graph = graph;
            this.root = graph.root;
        }

        graph.updateIdentifiers();
        if (graph.verifyIntegrity()) {
            var g = new dagre.graphlib.Graph();
            g.setGraph({
                
            });
            g.setDefaultEdgeLabel(() => ({}));

            graph.nodes.forEach((node) => {
                let {width, height} = this.vertexRenderer.vertexSize(node, ctx);
                g.setNode(node.id.toString(), {width: width, height: height});
            });

            graph.edges.forEach((edge) => {
                console.log(edge);
                g.setEdge(edge.source.id.toString(), edge.target.id.toString());
            });

            dagre.layout(g);
            g.nodes().forEach((v) => {
                let node = graph.nodes.find((n) => n.id == v);
                let nodeInfo = g.node(v);
                let nodeX = nodeInfo.x;
                let nodeY = nodeInfo.y;

                let {width, height} = this.vertexRenderer.vertexSize(node, ctx);
                this.drawingData.vertexData.push({
                    vertex: node,
                    x: nodeX,
                    y: nodeY,
                    width: width,
                    height: height
                });
            });

            g.edges().forEach((e) => {
                let points = g.edge(e);
                let edge = graph.edges.find((edge) => edge.source.id == parseInt(e.v) && edge.target.id == parseInt(e.w));
                let source = this.drawingData.vertexData.find((v) => v.vertex.id == edge.source.id);
                let target = this.drawingData.vertexData.find((v) => v.vertex.id == edge.target.id);

                this.edgeRenderer.render(source, target, widget, ctx);
                this.drawingData.edgeData.push({source: source, target: target});
            });

            g.nodes().forEach((v) => {
                let node = graph.nodes.find((n) => n.id == v);
                let nodeInfo = g.node(v);
                let nodeX = nodeInfo.x;
                let nodeY = nodeInfo.y;

                this.vertexRenderer.render(node, widget, ctx, nodeX, nodeY)
            });
        } else {
            console.log("temporary error message");
        }
    }
}

class BoxEdgeRenderer extends EdgeRenderer {
    constructor() {
        super();
        this.lineSize = 2;
        this.drawnEdges = []; // TODO: Fix stupid paths
        // I can probably fix this by pre-sending all the edges to the renderer, then telling the renderer to 
        // draw them all at once after the pre-processing is done to determine how to spread each start point and end point
        // alone the top of the source, and the bottom of the target.
        this.lineColor = "#9b9b9b";
    }

    render(source, target, widget, ctx) {
        ctx.strokeStyle = this.lineColor;
        ctx.lineWidth = this.lineSize;
        ctx.beginPath();
        
        let sourceX = source.x;
        let sourceY = source.y + (source.height/2);

        let targetX = target.x;
        let targetY = target.y - (target.height/2);

        let distanceY = targetY - sourceY;

        ctx.moveTo(sourceX, sourceY);
        ctx.lineTo(sourceX, sourceY + (distanceY/2));
        ctx.moveTo(sourceX, sourceY + (distanceY/2));
        ctx.lineTo(targetX, sourceY + (distanceY/2));
        ctx.moveTo(targetX, sourceY + (distanceY/2));
        ctx.lineTo(targetX, targetY);

        ctx.stroke();
    }
}

class BoxVertexRenderer extends VertexRenderer {
    constructor() {
        super();

        this.size = 16;
        this.linePadding = 5;
        this.borderSize = 1;
        this.boxPadding = {
            x: 10,
            y: 20
        }
        this.boxShadow = 1;

        this.shadowOffset = 4;
        this.shadowBlur = 0;

        this.boxColors = {
            border: "#9b9b9b",
            background: "#0f0f0f",
            shadow: "#080808"
        }
    }

    longestText(ctx, text) {
        let longest = null;
        text.forEach((t) => {
            let metrics = ctx.measureText(t.raw());
            if (!longest || metrics.width > longest.width) {
                longest = metrics;
            }
        });
        return longest;
    }

    vertexSize(vertex, ctx) {
        ctx.font = this.size + "px Consolas";
        let metrics = this.longestText(ctx, vertex.content);
        let textHeight = this.textHeight(vertex, ctx);

        let width = metrics.width + (this.boxPadding.x * 2);
        let height = ((textHeight + this.linePadding) * (vertex.content.length-1)) + (this.boxPadding.y * 2);
        return {width, height};
    }

    textHeight(vertex, ctx) {
        ctx.font = this.size + "px Consolas";
        let metrics = this.longestText(ctx, vertex.content);
        return metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    }

    renderContents(vertex, ctx, vertexX, vertexY) {
        let textHeight = this.textHeight(vertex, ctx);
        // draw each line's text respecting padding and line height
        vertex.content.forEach((l, i) => {
            if (l instanceof Line) {
                let nextX = vertexX + this.boxPadding.x;
                l.content.forEach((t, j) => {
                    ctx.fillStyle = t.color;
                    ctx.fillText(
                        t.content, 
                        nextX, 
                        vertexY + this.boxPadding.y
                            + (i * textHeight) 
                            + (this.linePadding * (i+1))
                    );
                    nextX += ctx.measureText(t.content).width;
                });
            }
        });
    }

    renderBox(widget, ctx, vertexX, vertexY, width, height) {
        ctx.shadowColor = this.boxColors.shadow;
        ctx.shadowOffsetX = (this.shadowOffset) * widget.camera.zoom;
        ctx.shadowOffsetY = (this.shadowOffset) * widget.camera.zoom;
        ctx.shadowBlur = this.shadowBlur;

        ctx.fillStyle = this.boxColors.border;
        ctx.fillRect(
            vertexX - this.borderSize, 
            vertexY - this.borderSize, 
            width + (this.borderSize*2), 
            height + (this.borderSize*2)
        );

        ctx.shadowColor = "transparent";

        ctx.fillStyle = this.boxColors.background;
        ctx.fillRect(vertexX, vertexY, width, height);
    }
    
    render(vertex, widget, ctx, vertexX, vertexY) {
        // calculate width and height based on vertex.content
        let {width, height} = this.vertexSize(vertex, ctx);

        this.renderBox(widget, ctx, vertexX-(width/2), vertexY-(height/2), width, height);
        this.renderContents(vertex, ctx, vertexX-(width/2), vertexY-(height/2));

        return {vertex: vertex, x: vertexX, y: vertexY, width: width, height: height};
    }
}