class TextEditorWidget extends Widget {
    static Config() {
        return TextEditorConfig.Configs[TextEditorWidget];
    }

    constructor(title, width, height) {
        super(title, width, height);

        this.bottom = this.element.appendChild(document.createElement('div'));
        this.bottom.classList.add('bottom-bar');

        this.editor = this.element.appendChild(document.createElement('div'));
        this.editor.classList.add('text-editor');
        this.editor.style.width = '100%';
        this.editor.style.height = 'calc(100% - 42px)';
        this.editor.setAttribute('spellcheck', 'false');
        this.editor.setAttribute('autocomplete', 'off');
        this.editor.setAttribute('autocorrect', 'off');
        this.editor.setAttribute('autocapitalize', 'off');

        this.margin = this.editor.appendChild(document.createElement('div'));
        this.margin.classList.add('margin');

        this.text = this.editor.appendChild(document.createElement('textarea'));
        this.text.classList.add('text-area');

        this.text.addEventListener('input', () => {
            this.lines = this.text.value.split('\n');
            this.refresh();
            this.selectCurrentLine();
        });

        this.text.addEventListener('keydown', (e) => {
            if (e.keyCode === 9) {
                // tab insertion
                e.preventDefault();

                let cursorPosition = $(this.text).prop('selectionStart');
                let start = this.text.value.substr(0, cursorPosition);
                let end = this.text.value.substr(cursorPosition);

                this.text.value = start + '\t' + end;

                this.lines = this.text.value.split('\n');
                this.refresh();
                this.selectCurrentLine();

                this.text.setSelectionRange(cursorPosition + 1, cursorPosition + 1);
            }

            // for some reason, the cursor position is not updated when the keydown event is fired
            // so we need to wait a bit before selecting the current line
            this.selectCurrentLine();
        });

        let margin = this.margin;
        $(this.text).scroll(function() {
            margin.scrollTop = this.scrollTop;
        });

        $(this.text).click(this.selectCurrentLine.bind(this));

        this.onExpand['editor'] = () => {
            this.editor.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            this.bottom.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            if (this.flags.expanded) {
                this.lines = this.text.value.split('\n');
                this.refresh();
                this.selectCurrentLine();
            }
        }
    }

    selectCurrentLine() {
        setTimeout(() => {
            let cursorPosition = $(this.text).prop('selectionStart');
            let line = $(this.text).val().substr(0, cursorPosition).split('\n').length;
    
            $('.line-number').removeClass('active-line-number');
            this.margin.childNodes[line - 1].childNodes[0].classList.add('active-line-number');
            let scrollTop = $(this.text).scrollTop();
            this.margin.scrollTop = scrollTop;
        }, 10);
    }

    refresh() {
        if (this.margin.children.length < this.lines.length) {
            for (let i = this.margin.children.length; i < this.lines.length; i++) {
                const lineMargin = this.margin.appendChild(document.createElement('div'));
                lineMargin.classList.add('line-margin');

                const lineNumber = lineMargin.appendChild(document.createElement('div'));
                lineNumber.classList.add('line-number');
                lineNumber.innerText = i + 1;
            }
        } else if (this.margin.children.length > this.lines.length) {
            for (let i = this.margin.children.length; i > this.lines.length; i--) {
                this.margin.removeChild(this.margin.lastChild);
            }
        }
    }
}