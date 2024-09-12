async function categoryExpandOnClick(e) {
    let categoryElement = $(this);

    if (categoryElement.hasClass('toolbar-category-expand')) {
        categoryElement = categoryElement.parent();
    }

    if (!$(this).hasClass('toolbar-category-selected')) {
        // wait for toolbar-category-dropdown to be created
        var check = setInterval(function() {
            if ($('.toolbar-category-dropdown').length) {
                var dropdown = categoryElement.find('.toolbar-category-dropdown');
                
                if (categoryElement.parent().attr('id') == 'toolbar') {
                    var rect = categoryElement[0].getBoundingClientRect();
                    var toolbarRect = document.getElementById('toolbar').getBoundingClientRect();
                    
                    dropdown.css('left', rect.left + 'px');
                    dropdown.css('top', toolbarRect.bottom + 'px');
                } else {
                    var parentDropdown = categoryElement.parent();
                    var rect = parentDropdown[0].getBoundingClientRect();
        
                    dropdown.css('left', rect.width + 'px');
                }

                // get all child elements of the toolbar-category-dropdown that are
                // toolbar-category
                var components = dropdown.children('.toolbar-category');
                for (var i = 0; i < components.length; i++) {
                    var component = $(components[i]);
                    let expand = document.createElement('div');

                    expand.classList.add('toolbar-category-expand');
                    expand.innerText = '';

                    //expand.addEventListener('click', categoryExpandOnClick); // TODO: Fix
                    $(document).on('click', '.toolbar-category-expand', categoryExpandOnClick);
                    component.append(expand);
                }

                clearInterval(check);
            }
        }, 10);
    }

    e.stopPropagation();
}

$(document).on('click', '.toolbar-category', categoryExpandOnClick);