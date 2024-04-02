// https://www.cs.unc.edu/techreports/89-034.pdf
class BoxRenderer extends GraphRenderer {
    constructor() {
        super();

        this.graph = null;
        this.root = null;

        this.vertexRenderer = new BoxVertexRenderer();
        this.edgeRenderer = new BoxEdgeRenderer();
        this.drawingData = {
            vertexData: [],
            edgeData: []
        }
    }

    preprocess(ctx, data) {
        let updateIfNewGraph = (graph) => {
            if (this.graph != graph) {
                this.graph = graph;
                this.root = graph.root;
                this.drawingData = { vertexData: [], edgeData: [] };
                this.vertexRenderer.drawingData = [];
                this.edgeRenderer.drawingData = [];
                this.graph.updateIdentifiers();
            }
        };

        let preprocessGraph = () => {
            if (this.graph.verifyIntegrity()) {
                var g = new dagre.graphlib.Graph();
                g.setGraph({});
                g.setDefaultEdgeLabel(() => ({}));
    
                this.graph.nodes.forEach((node) => {
                    let metrics = this.vertexRenderer.metrics(ctx, node);
                    let width = metrics.vertex.width;
                    let height = metrics.vertex.height;

                    g.setNode(node.id.toString(), {width: width, height: height});
                });
    
                this.graph.edges.forEach((edge) => {
                    g.setEdge(edge.source.id.toString(), edge.target.id.toString());
                });
    
                dagre.layout(g);
                return g;
            }
            return null;
        };

        let preprocessVertices = (g) => {
            if (this.drawingData.vertexData.length > 0) return;

            g.nodes().forEach((v) => {
                let node = this.graph.nodes.find((n) => n.id == v);
                let nodeInfo = g.node(v);
                let nodeX = nodeInfo.x;
                let nodeY = nodeInfo.y;

                let metrics = this.vertexRenderer.metrics(ctx, node);
                let width = metrics.vertex.width;
                let height = metrics.vertex.height;

                this.drawingData.vertexData.push({
                    vertex: node,
                    x: nodeX,
                    y: nodeY,
                    width: width,
                    height: height
                });
            });

            this.vertexRenderer.preprocess(ctx, this.drawingData.vertexData);
        };

        let preprocessEdges = (g) => {
            if (this.drawingData.edgeData.length > 0) return;
            g.edges().forEach((e) => {
                let points = g.edge(e);
                let edge = this.graph.edges.find((edge) => edge.source.id == parseInt(e.v) && edge.target.id == parseInt(e.w));
                let source = this.drawingData.vertexData.find((v) => v.vertex.id == edge.source.id);
                let target = this.drawingData.vertexData.find((v) => v.vertex.id == edge.target.id);
                this.drawingData.edgeData.push({source: source, target: target});
            });

            this.edgeRenderer.preprocess(ctx, this.drawingData.edgeData);
        };

        updateIfNewGraph(data);
        let g = preprocessGraph(this.graph);
        if (g) {
            preprocessVertices(g);
            preprocessEdges(g);
        } else {
            // TODO: Prepare ErrorRenderer
        }
    }

    render(ctx, widget) {
        this.edgeRenderer.render(ctx, widget);
        this.vertexRenderer.render(ctx, widget);
    }
}

class BoxEdgeRenderer extends EdgeRenderer {
    constructor() {
        super();
        this.vertexInfo = [];
    }

    preprocess(ctx, data) {
        data.forEach((edge) => {
            let source = edge.source;

            // (* check if the source is already in the vertex info array *)
            let sourceInfo = this.vertexInfo.find((info) => info.source.vertex.id == source.vertex.id);
            
            // (* if the source is not in the vertex info array, add it *)
            if (!sourceInfo) {
                sourceInfo = {
                    source: source,
                    sourceMidX: source.x,
                    sourceBottomY: source.y + (source.height/2),

                    targets: [
                        {
                            target: edge.target,
                            targetMidX: edge.target.x,
                            targetTopY: edge.target.y - (edge.target.height/2)
                        }
                    ]
                };

                this.vertexInfo.push(sourceInfo);
            } else {
                sourceInfo.targets.push({
                    target: edge.target,
                    targetMidX: edge.target.x,
                    targetTopY: edge.target.y - (edge.target.height/2)
                });
            }
        });
    }

    render(ctx, widget) {
        let drawPoint = (x, y) => { // for debugging
            ctx.strokeStyle = EdgeRenderer.Config().COLORS.COLOR_DIRECT;
            ctx.beginPath();
            ctx.arc(x, y, 10, 0, 2 * Math.PI);
            ctx.stroke();
        };

        let drawLine = (startX, startY, endX, endY, color) => {
            // TODO: make it avoid going through vertices, also adjust size
            let distanceY = endY - startY;

            ctx.strokeStyle = color;
            ctx.lineWidth = EdgeRenderer.Config().SIZES.SIZE_LINE;
            ctx.beginPath();

            ctx.moveTo(startX, startY);
            ctx.lineTo(startX, startY + (distanceY/2));
            ctx.moveTo(startX, startY + (distanceY/2));
            ctx.lineTo(endX, startY + (distanceY/2));
            ctx.moveTo(endX, startY + (distanceY/2));
            ctx.lineTo(endX, endY);

            ctx.stroke();
        };

        this.vertexInfo.forEach((sourceInfo) => {
            let numTargets = sourceInfo.targets.length;
            let remainder = numTargets % 2;

            if (numTargets == 1) {
                let target = sourceInfo.targets[0];
                drawLine(
                    sourceInfo.sourceMidX, 
                    sourceInfo.sourceBottomY, 
                    target.targetMidX, 
                    target.targetTopY, 
                    EdgeRenderer.Config().COLORS.COLOR_DIRECT
                );
            } else {
                let evened = numTargets - remainder;
                let half = (evened / 2);

                let sourceX = sourceInfo.sourceMidX;
                let sourceY = sourceInfo.sourceBottomY;

                let left = sourceX - (sourceInfo.source.width/2);
                let right = sourceX + (sourceInfo.source.width/2);

                let padding = sourceInfo.source.width * EdgeRenderer.Config().PADDING.PADDING_BETWEEN_EDGES;
                sourceX -= (padding * half);

                for (let i = 0; i < half; i++) {
                    let target = sourceInfo.targets[i];
                    drawLine(
                        sourceX, 
                        sourceY, 
                        target.targetMidX, 
                        target.targetTopY, 
                        EdgeRenderer.Config().COLORS.COLOR_TRUE
                    );
                    sourceX += padding;
                }

                sourceX = sourceInfo.sourceMidX;
                if (remainder == 1) {
                    let target = sourceInfo.targets[half];
                    drawLine(
                        sourceX, 
                        sourceY, 
                        target.targetMidX, 
                        target.targetTopY, 
                        EdgeRenderer.Config().COLORS.COLOR_DIRECT
                    );
                }
                
                sourceX += (padding * half);
                for (let i = half; i < numTargets; i++) {
                    let target = sourceInfo.targets[i];
                    drawLine(
                        sourceX, 
                        sourceY, 
                        target.targetMidX, 
                        target.targetTopY, 
                        EdgeRenderer.Config().COLORS.COLOR_FALSE
                    );
                    sourceX -= padding;
                }
            }
        });
    }
}

class BoxVertexRenderer extends VertexRenderer {
    constructor() {
        super();
        this.drawingData = [];
    }

    metrics(ctx, vertex) {
        // (* calculate content height in px, and the longest width line in content *)
        let metrics = { 
            content: {
                fontHeight: 0, maximumWidth: 0, widestLine: null 
            },
            vertex: {
                width: 0, height: 0
            }
        };

        // (* set appropriate font size and family according to config *)
        ctx.font = VertexRenderer.Config().SIZES.SIZE_TEXT 
            + "px "
            + VertexRenderer.Config().FONTS.FONT_CONTENT;

        // (* get the widest line in the content provided *)
        vertex.content.forEach((line) => {
            let width = 0;
            line.content.forEach((text) => {
                width += ctx.measureText(text.content).width;
            });

            if (metrics.content.widestLine == null || width > metrics.content.maximumWidth) {
                metrics.content.widestLine = ctx.measureText(line.raw());
                metrics.content.maximumWidth = width;
            }
        });
        
        // (* calculate the height of the font *)
        metrics.content.fontHeight = metrics.content.widestLine.actualBoundingBoxAscent 
            + metrics.content.widestLine.actualBoundingBoxDescent;

        // (* retrieve the width of the widest line *)
        metrics.content.maximumWidth = metrics.content.widestLine.width;

        // (* calculate the width and height of the vertex *)
        metrics.vertex.width = metrics.content.maximumWidth + (VertexRenderer.Config().PADDING.PADDING_BOX_HORIZONTAL * 2);
        metrics.vertex.height = ((metrics.content.fontHeight + VertexRenderer.Config().PADDING.PADDING_LINE) 
            * (vertex.content.length - 1)) 
            + (VertexRenderer.Config().PADDING.PADDING_BOX_VERTICAL * 2);
        
        // (* return the metrics *)
        return metrics;
    }

    preprocess(ctx, data) {
        let preprocessContent = (startX, startY, metrics, content) => {
            let lines = [];
            let textHeight = metrics.content.fontHeight;
            let width = metrics.vertex.width;
            let height = metrics.vertex.height;

            // (* draw each line's text respecting padding and line height *)
            content.forEach((line, i) => {
                // (* calculate the y position of the line *)
                let y = startY 
                    + VertexRenderer.Config().PADDING.PADDING_BOX_VERTICAL 
                    + (i * textHeight) 
                    + (VertexRenderer.Config().PADDING.PADDING_LINE * (i+1));

                let lineData = {y: y, content: []};
                
                if (line instanceof Line) {
                    // (* calculate the x position of the line *)
                    let x = startX + VertexRenderer.Config().PADDING.PADDING_BOX_HORIZONTAL;

                    line.content.forEach((text, j) => {
                        // (* push the text data to the line data *)
                        lineData.content.push([
                            text.color, 
                            text.content, 
                            x
                        ]);

                        // (* increment the x position *)
                        x += ctx.measureText(text.content).width;
                    });
                }
                
                // (* push the line data to the lines array *)
                lines.push(lineData);
            });

            // (* return the lines *)
            return lines;
        };

        let preprocessVertex = (vertex) => {
            // (* calculate the metrics for the vertex *)
            let metrics = this.metrics(ctx, vertex.vertex);
            let x = vertex.x;
            let y = vertex.y;

            // (* center the vertex if the config is set to do so *)
            if (VertexRenderer.Config().RENDERING.VERTEX_CENTERING) {
                x -= metrics.vertex.width / 2;
                y -= metrics.vertex.height / 2;
            }

            // (* preprocess the content of the vertex *)
            let lines = preprocessContent(x, y, metrics, vertex.vertex.content);

            // (* push the data to the drawing data *)
            this.drawingData.push({
                dagre_vertex: vertex,
                vertex: vertex.vertex,

                x: x,
                y: y,
                
                lines: lines,
                metrics: metrics
            });
        }

        data.forEach((vertex) => {
            // (* preprocess each vertex *)
            preprocessVertex(vertex);
        });
    }

    render(ctx, widget) {
        this.drawingData.forEach((vertex) => {
            { // (* draw the box for the vertex *)
                // (* set the shadow properties *)
                ctx.shadowColor = VertexRenderer.Config().COLORS.COLOR_SHADOW;
                ctx.shadowOffsetX = (VertexRenderer.Config().OFFSETS.OFFSET_SHADOW) * widget.camera.zoom;
                ctx.shadowOffsetY = (VertexRenderer.Config().OFFSETS.OFFSET_SHADOW) * widget.camera.zoom;
                ctx.shadowBlur = VertexRenderer.Config().SIZES.SIZE_SHADOW;

                // (* draw the border of the vertex *)
                ctx.fillStyle = VertexRenderer.Config().COLORS.COLOR_BORDER;
                ctx.fillRect(
                    vertex.x - VertexRenderer.Config().SIZES.SIZE_BORDER, 
                    vertex.y - VertexRenderer.Config().SIZES.SIZE_BORDER, 
                    vertex.metrics.vertex.width + (VertexRenderer.Config().SIZES.SIZE_BORDER*2), 
                    vertex.metrics.vertex.height + (VertexRenderer.Config().SIZES.SIZE_BORDER*2)
                );

                // (* set the shadow color to transparent *)
                ctx.shadowColor = "transparent";

                // (* draw the background of the vertex *)
                ctx.fillStyle = VertexRenderer.Config().COLORS.COLOR_BACKGROUND;
                ctx.fillRect(vertex.x, vertex.y, vertex.metrics.vertex.width, vertex.metrics.vertex.height);
            }

            { // (* draw the text for the vertex *)
                // (* set the font properties *)
                ctx.font = VertexRenderer.Config().SIZES.SIZE_TEXT 
                    + "px " 
                    + VertexRenderer.Config().FONTS.FONT_CONTENT;
                
                // (* draw each line of text *)
                vertex.lines.forEach((line) => {
                    line.content.forEach((text) => {
                        ctx.fillStyle = text[0];
                        ctx.fillText(text[1], text[2], line.y);
                    });
                });
            }
        });
    }
}