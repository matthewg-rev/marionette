toolbar = new Toolbar(document.getElementById('toolbar'));
ToolCanvas = new Canvas();

file = new ToolbarCategory('File');
{
    file.children.push(new ToolbarTool('Exit', function() {
        window.close();
    }));
}

view = new ToolbarCategory('View');
{
    data = new ToolbarCategory('Data');
    data.children.push(new ToolbarTool('Strings', function() {}));
    view.children.push(data);
}

{
    debug = new ToolbarCategory('Debug');
    debug.children.push(new ToolbarTool('Breakpoints', function() {}));
    view.children.push(debug);
}

{
    view.children.push(new ToolbarTool('Graph View', function() {
        ToolCanvas.addWidget(new GraphWidget('Graph', 601, 400));
    }));
    view.children.push(new ToolbarTool('Clock View', function() {
        ToolCanvas.addWidget(new ClockWidget('Clock', 201, 200));
    }));
}

toolbar.components.push(file);
toolbar.components.push(view);
toolbar.populate();

ToolCanvas.addWidget(new GraphWidget('Graph', 601, 400));