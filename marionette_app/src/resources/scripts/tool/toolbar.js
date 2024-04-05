class Toolbar {
    constructor(element) {
        this.bar = element;
        this.current = null;
        this.components = [];
    }

    populate() {
        for (let category of this.components) {
            let categoryElement = document.createElement('div');
            let originalHeight = categoryElement.style.height;
            
            category.parent = this;
            categoryElement.classList.add('toolbar-category');
            categoryElement.innerText = category.name;
            categoryElement.addEventListener('click', function(e) {
                category.expanded = !category.expanded;
                if (category.expanded) {
                    categoryElement.style.height = originalHeight;
                    categoryElement.classList.add('toolbar-category-selected');
                    category.populate();
                } else {
                    categoryElement.classList.remove('toolbar-category-selected');
                    categoryElement.style.height = 'auto';
                    category.clear();
                }
                e.stopPropagation();
            });

            this.bar.appendChild(categoryElement);
            category.setElement(categoryElement);
            category.setToolbar(this.bar);
        }
    
    }
}

class ToolbarCategory {
    constructor(name) {
        this.name = name;

        this.children = [];
        this.expanded = false;
        this.current = null;

        this.parent = null;
        this.element = null;
        this.toolbar = null;
    }

    setElement(element) {
        this.element = element;
    }

    setToolbar(toolbar) {
        this.toolbar = toolbar;
    }

    populate() {
        let dropdown = document.createElement('div');
        dropdown.classList.add('toolbar-category-dropdown');
        if (this.parent.components !== undefined) {
            // center the dropdown under the element
            let rect = this.element.getBoundingClientRect();
            let toolbarRect = this.toolbar.getBoundingClientRect();
            dropdown.style.left = rect.left + 'px';
            dropdown.style.top = toolbarRect.bottom + 'px';
        } else {
            // get absolute width of the parent element
            let rect = this.parent.element.getBoundingClientRect();
            dropdown.style.left = rect.right + 'px';
        }
        this.element.appendChild(dropdown);

        for (let tool of this.children) {
            let toolElement = document.createElement('div');
            toolElement.classList.add('toolbar-tool');
            toolElement.innerText = tool.name;

            tool.setElement(toolElement);
            if (tool.children !== undefined) {
                let expand = document.createElement('div');
                expand.classList.add('toolbar-tool-expand');
                expand.innerText = '';

                let expandFn = function(e) {
                    tool.expanded = !tool.expanded;
                    if (tool.expanded) {
                        tool.populate();
                    } else {
                        tool.clear();
                    }
                    e.stopPropagation();
                };
                expand.addEventListener('click', expandFn);
                toolElement.addEventListener('click', expandFn);
                toolElement.appendChild(expand);
                tool.parent = this;
            } else {
                toolElement.addEventListener('click', tool.onClick);
            }
            dropdown.appendChild(toolElement);
        }
    }

    clear() {
        for (let tool of this.children) {
            tool.remove();
        }
        this.element.querySelector('.toolbar-category-dropdown').remove();
    }

    remove() {
        this.element.remove();
    }
}

class ToolbarTool {
    constructor(name, onClick) {
        this.name = name;
        this.onClick = onClick;
    }

    setElement(element) {
        this.element = element;
    }

    remove() {
        this.element.remove();
    }
}