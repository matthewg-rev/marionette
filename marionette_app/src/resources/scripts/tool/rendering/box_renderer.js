class BoxRenderer extends GraphRenderer {
    constructor(camera) {
        super(camera);

        this.graph = null;
        this.root = null;

        this.vertexRenderer = new BoxVertexRenderer(this.camera);
        this.edgeRenderer = new BoxEdgeRenderer(this.camera);
        this.drawingData = {
            vertexData: [],
            edgeData: []
        }
    }

    drawPoint(ctx, x, y) { // for debugging
        ctx.strokeStyle = "#ff0000";
        ctx.beginPath();
        ctx.arc(x, y, 10, 0, 2 * Math.PI);
        ctx.stroke();
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

    select(ctx, location) {
        // (* find any vertex that is selected                          *)
        let vertex = this.drawingData.vertexData.find((vertex) => {
            return vertex.vertex.selected;
        });
        
        // (* if a vertex is selected, deselect it                      *)
        if (vertex) {
            vertex.vertex.selected = false;
        }

        // (* find any vertex that contains the location                *)
        vertex = this.drawingData.vertexData.find((vertex) => {
            // (* vertex.x and vertex.y are the midpoints of the vertex *)
            let x = vertex.x;
            let y = vertex.y;
            let width = vertex.width;
            let height = vertex.height;
            let x1 = x - (width/2);
            let x2 = x + (width/2);
            let y1 = y - (height/2);
            let y2 = y + (height/2);

            return location.x >= x1 && location.x <= x2 && location.y >= y1 && location.y <= y2;
        });

        // (* if a vertex is found, select it                           *)
        if (vertex) {
            vertex.vertex.selected = true;
            return vertex.vertex;
        }

        // (* return null if no vertex is found                         *)
        return null;
    }

    render(ctx, widget) {
        this.edgeRenderer.render(ctx, widget);
        this.vertexRenderer.render(ctx, widget);   
    }
}

class BoxEdgeRenderer extends EdgeRenderer {
    constructor(camera) {
        super(camera);
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
                            targetMidY: edge.target.y,

                            targetTopY: edge.target.y - (edge.target.height/2),
                            targetBottomY: edge.target.y + (edge.target.height/2)
                        }
                    ]
                };

                this.vertexInfo.push(sourceInfo);
            } else {
                sourceInfo.targets.push({
                    target: edge.target,

                    targetMidX: edge.target.x,
                    targetMidY: edge.target.y,

                    targetTopY: edge.target.y - (edge.target.height/2),
                    targetBottomY: edge.target.y + (edge.target.height/2)
                });
            }
        });
    }

    render(ctx, widget) {
        const config = EdgeRenderer.Config();
        const drawLine = (source, target, startX, startY, endMidX, endMidY, color) => {
            const linePadding = config.PADDING.PADDING_LINE;
            const above = startY > endMidY;
            const left = startX > endMidX;
            const right = startX < endMidX;
            const endY = above ? target.targetTopY : target.targetBottomY;

            ctx.strokeStyle = color;
            ctx.lineWidth = config.SIZES.SIZE_LINE;
            ctx.beginPath();

            const bendStartY = startY + linePadding;
            ctx.moveTo(startX, startY);
            ctx.lineTo(startX, bendStartY);

            if (above) {
                const horizontalDistance = (source.x + (source.width / 2)) - startX;
                let newX = startX + (left ? horizontalDistance : -horizontalDistance);

                ctx.lineTo(newX, bendStartY);

                newX += horizontalDistance + linePadding;
                const verticalY = endY - linePadding;
                ctx.lineTo(newX, verticalY);
                ctx.lineTo(endMidX, verticalY);
            } else {
                ctx.lineTo(endMidX, bendStartY);
            }

            ctx.lineTo(endMidX, endY);
            ctx.stroke();
        };

        this.vertexInfo.forEach((sourceInfo) => {
            const numTargets = sourceInfo.targets.length;
            const remainder = numTargets % 2;
            const evened = numTargets - remainder;
            const half = evened / 2;

            let sourceX = sourceInfo.sourceMidX - (sourceInfo.source.width * config.PADDING.PADDING_BETWEEN_EDGES * half);
            const sourceY = sourceInfo.sourceBottomY;

            sourceInfo.targets.sort((a, b) => a.targetMidX - b.targetMidX);

            sourceInfo.targets.forEach((target, index) => {
                const color = index < half ? config.COLORS.COLOR_TRUE : (index >= numTargets - half ? config.COLORS.COLOR_FALSE : config.COLORS.COLOR_DIRECT);

                drawLine(
                    sourceInfo.source,
                    target,
                    sourceX,
                    sourceY,
                    target.targetMidX,
                    target.targetMidY,
                    color
                );
                sourceX += sourceInfo.source.width * config.PADDING.PADDING_BETWEEN_EDGES;
            });
        });
    }
}

class BoxVertexRenderer extends VertexRenderer {
    constructor(camera) {
        super(camera);
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
        const config = VertexRenderer.Config();
        const shadowOffsetX = config.OFFSETS.OFFSET_SHADOW * widget.camera.zoom;
        const shadowOffsetY = shadowOffsetX; // Assuming symmetric shadow offsets

        this.drawingData.forEach((vertex) => {
            const isSelected = vertex.vertex.selected;
            const shadowColor = isSelected ? config.COLORS.COLOR_SHADOW_SELECTED : config.COLORS.COLOR_SHADOW;
            const borderColor = isSelected ? config.COLORS.COLOR_BORDER_SELECTED : config.COLORS.COLOR_BORDER;
            const borderSize = config.SIZES.SIZE_BORDER;
            const vertexWidth = vertex.metrics.vertex.width;
            const vertexHeight = vertex.metrics.vertex.height;

            // Set shadow properties
            ctx.shadowBlur = config.SIZES.SIZE_SHADOW_BLUR;
            ctx.shadowColor = shadowColor;
            ctx.shadowOffsetX = shadowOffsetX;
            ctx.shadowOffsetY = shadowOffsetY;
            ctx.shadowBlur = config.SIZES.SIZE_SHADOW;

            // Draw the border of the vertex
            ctx.fillStyle = borderColor;
            ctx.fillRect(
                vertex.x - borderSize, 
                vertex.y - borderSize, 
                vertexWidth + (borderSize * 2), 
                vertexHeight + (borderSize * 2)
            );

            // Set the shadow color to transparent for the background
            ctx.shadowColor = "transparent";

            // Draw the background of the vertex
            ctx.fillStyle = config.COLORS.COLOR_BACKGROUND;
            ctx.fillRect(vertex.x, vertex.y, vertexWidth, vertexHeight);

            // Draw the text for the vertex
            ctx.font = `${config.SIZES.SIZE_TEXT}px ${config.FONTS.FONT_CONTENT}`;
            vertex.lines.forEach((line) => {
                line.content.forEach((text) => {
                    ctx.fillStyle = text[0];
                    ctx.fillText(text[1], text[2], line.y);
                });
            });
        });
    }
}