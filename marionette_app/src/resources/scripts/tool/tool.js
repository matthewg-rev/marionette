toolbar = new Toolbar(document.getElementById('toolbar'));
ToolCanvas = new Canvas();

file = new ToolbarCategory('File');
{
    file.components.push(new ToolbarTool('Exit', function() {
        window.close();
    }));
}

widgets = new ToolbarCategory('Widgets');
{
    {
        analysis = new ToolbarCategory('Analysis');

        analysis.components.push(new ToolbarTool('Graph View', function() {
            ToolCanvas.addWidget(new GraphWidget('Graph View', 601, 400));
        }));

        analysis.components.push(new ToolbarTool('Text View', function() {
            ToolCanvas.addWidget(new TextEditorWidget('Text View', 400, 400));
        }));

        widgets.components.push(analysis);
    }
    {
        misc = new ToolbarCategory('Miscellaneous');
        misc.components.push(new ToolbarTool('Clock View', function() {
            ToolCanvas.addWidget(new ClockWidget('Clock', 201, 200));
        }));
        widgets.components.push(misc);
    }
}

toolbar.components.push(file);
toolbar.components.push(widgets);
toolbar.create();