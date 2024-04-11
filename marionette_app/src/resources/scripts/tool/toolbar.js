class Toolbar {
    constructor(element) {
        this.bar = element;
        this.current = null;
        this.components = [];
    }

    selected(item) {
        let path = [];
        let current = item;
        while (current.parent !== undefined) {
            path.push(current);
            current = current.parent;
        }
        path.push(this);

        for (let node of path) {
            for (let component of node.components) {
                if (path.indexOf(component) === -1) {
                    if (component.closed !== undefined) {
                        component.closed();
                    }
                }
            }
        }
    }

    hideEverything(item) {
        let path = [];
        let current = item;
        while (current.parent !== undefined) {
            path.push(current);
            current = current.parent;
        }
        path.push(this);

        for (let node of path) {
            for (let component of node.components) {
                component.closed();
            }
        }
    }

    create() {
        for (let category of this.components) {
            category.setToolbar(this);
            category.create(this);
            this.bar.appendChild(category.element);
        }
    }
}

class ToolbarCategory {
    constructor(name) {
        this.name = name;

        this.components = [];
        this.expanded = false;

        this.parent = null;
        this.element = null;
        this.toolbar = null;
    }

    setToolbar(toolbar) {
        this.toolbar = toolbar;
    }

    create(parent) {
        let categoryElement = document.createElement('div');
        let originalHeight = categoryElement.style.height;

        categoryElement.classList.add('toolbar-category');
        categoryElement.innerText = this.name;

        // this is such a poor decision on behalf of the javascript
        // standard, why is this required for me to access `this`
        let category = this; 
        let onClick = function(e) {
            category.expanded = !category.expanded;
            if (category.expanded) {
                categoryElement.style.height = originalHeight;
                categoryElement.classList.add('toolbar-category-selected');

                category.opened();
                category.toolbar.selected(category);
            } else {
                categoryElement.classList.remove('toolbar-category-selected');
                category.closed();
            }
            e.stopPropagation();
        }
        
        categoryElement.addEventListener('click', onClick);
        
        if (parent.bar === undefined) {
            let expand = document.createElement('div');
            expand.classList.add('toolbar-category-expand');
            expand.innerText = '';
            expand.addEventListener('click', onClick);
            categoryElement.appendChild(expand);
        }

        this.element = categoryElement;
        this.parent = parent;
    }

    opened() {
        this.expanded = true;

        let dropdown = document.createElement('div');
        this.element.appendChild(dropdown);
        dropdown.classList.add('toolbar-category-dropdown');

        if (this.parent.bar !== undefined) {
            // center the dropdown under the element
            let rect = this.element.getBoundingClientRect();
            let toolbarRect = this.toolbar.bar.getBoundingClientRect();
            dropdown.style.left = rect.left + 'px';
            dropdown.style.top = toolbarRect.bottom + 'px';
        } else {
            // get absolute width of the parent element
            let parentDropdown = this.parent.element.querySelector('.toolbar-category-dropdown');
            let rect = parentDropdown.getBoundingClientRect();
            dropdown.style.left = rect.width + 'px';
        }

        for (let component of this.components) {
            component.setToolbar(this.toolbar);
            if (component instanceof ToolbarCategory) {
                component.create(this);
            } else {
                component.setElement(document.createElement('div'));
                component.element.classList.add('toolbar-tool');
                component.element.innerText = component.name;
                component.setToolbar(this.toolbar);
                component.setParent(this);
                component.element.addEventListener('click', component.onClick);
            }
            dropdown.appendChild(component.element);
        }
    }

    closed() {
        this.expanded = false;
        this.element.classList.remove('toolbar-category-selected');
        let dropdown = this.element.querySelector('.toolbar-category-dropdown');

        if (dropdown !== null) {
            dropdown.remove();
        }
    }
}

class ToolbarTool {
    constructor(name, onClick) {
        this.name = name;

        let tool = this;
        this.onClick = function(e) {
            onClick();
            tool.toolbar.hideEverything(this);
            e.stopPropagation();
        }
    }

    setToolbar(toolbar) {
        this.toolbar = toolbar;
    }

    setParent(parent) {
        this.parent = parent;
    }

    setElement(element) {
        this.element = element;
    }

    remove() {
        this.element.remove();
    }
}