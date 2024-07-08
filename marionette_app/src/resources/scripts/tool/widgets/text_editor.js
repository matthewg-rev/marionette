class TextEditorWidget extends Widget {
    static Config() {
        return TextEditorConfig.Configs[TextEditorWidget];
    }

    constructor(title, width, height) {
        super(title, width, height);

        this.bottom = this.createElementWithClass('div', 'bottom-bar', this.element);
        this.editor = this.createElementWithClass('div', 'text-editor', this.element, {
            width: '100%',
            height: 'calc(100% - 42px)',
        }, {
            spellcheck: 'false',
            autocomplete: 'off',
            autocorrect: 'off',
            autocapitalize: 'off'
        });

        this.margin = this.createElementWithClass('div', 'margin', this.editor);
        this.container = this.createElementWithClass('div', 'container', this.editor);
        this.backdrop = this.createElementWithClass('div', 'backdrop', this.container);
        this.hilayer = this.createElementWithClass('div', 'hilayer', this.backdrop);
        this.text = this.createElementWithClass('textarea', 'text-area', this.container);

        this.current_linter_data = {
            "lines": [],
            "current_line": 0,
            "text": ""
        };
        
        $(this.text).on({
            'scroll': () => this.syncScrollPositions(),
            'input': () => this.onInput(),
            'keydown': (e) => this.keyDown(e)
        });

        $(this.text).click(this.selectCurrentLine.bind(this));

        this.onExpand['editor'] = () => {
            const visibility = this.flags.expanded ? 'visible' : 'hidden';
            this.editor.style.visibility = visibility;
            this.bottom.style.visibility = visibility;
            if (this.flags.expanded) {
                this.lines = this.text.value.split('\n');
                this.refresh();
                this.selectCurrentLine();
            }
        };
    }

    createElementWithClass(tag, className, parent, styles = {}, attributes = {}) {
        const element = document.createElement(tag);
        element.classList.add(className);
        Object.assign(element.style, styles);
        
        for (const [key, value] of Object.entries(attributes)) {
            element.setAttribute(key, value);
        }

        parent.appendChild(element);
        return element;
    }

    keyDown(e) {
        //this.syncScrollPositions()
        //this.refresh();

        if (e.key === 'Tab') {
            // tab insertion
            e.preventDefault();

            const cursorPosition = this.text.selectionStart;
            const start = this.text.value.substring(0, cursorPosition);
            const end = this.text.value.substring(cursorPosition);

            this.text.value = start + '\t' + end;
            this.text.setSelectionRange(cursorPosition + 1, cursorPosition + 1);
            this.onInput();
        }
    }

    async onInput() {
        this.syncScrollPositions()
        this.lines = this.text.value.split('\n');

        const cursorPosition = this.text.selectionStart;
        const lines = this.text.value.substr(0, cursorPosition).split('\n');
        const currentLineIndex = lines.length - 1;
        this.current_linter_data["current_line"] = currentLineIndex;

        try {
            /*let lintedText = this.text.value;
            let linterData = await window.internalRequest('lex', {"lexer":"python", "text": lintedText}, false, true);
            this.current_linter_data = linterData["data"];
            this.current_linter_text = lintedText;
            this.doLinting();*/
            
            let textToLint = this.text.value;
            let linterData = await window.internalRequest('lex', {"lexer":"python", "text": textToLint}, false, true);
            this.current_linter_data["lines"] = JSON.parse(linterData["data"]);
            this.current_linter_data["text"] = this.text.value;

            this.doLinting();
        } catch (error) {
            console.error("Error processing linter data:", error);
        }

        this.refresh();
        this.selectCurrentLine();
    }

    setReadOnly(readOnly) {
        this.text.readOnly = readOnly;
    }

    syncScrollPositions() {
        this.margin.scrollTop = this.text.scrollTop;
        this.backdrop.scrollTop = this.text.scrollTop;
        this.hilayer.scrollTop = this.text.scrollTop;

        this.backdrop.scrollLeft = this.text.scrollLeft;
        this.hilayer.scrollLeft = this.text.scrollLeft;
    }

    doLinting() {
        // TODO: fix this solution as it's awfully slow since we have to wait for dioxus to return linting results.
        let lintedText = this.current_linter_data["text"];
        let edits = [];

        for (let i = 0; i < this.current_linter_data["lines"].length; i++) {
            let tok = this.current_linter_data["lines"][i];
            let slice = tok["slice"];
            let span = tok["span"];
            let start = span["start"];
            let end = span["end"];
            let token = tok["token"];

            let editContent = token === "NewLine" ? `</div><div class='line'>` : `<span class='${token.toLowerCase()}'>${slice}</span>`;
            edits.push({
                start: start,
                end: end,
                edit: editContent,
                token: token
            });
        }

        edits.sort((a, b) => a.start - b.start);
        
        let lineAppendage = `<div class='line'>`;
        let addedCharacters = 0;
        lintedText = lineAppendage + lintedText;
        addedCharacters += lineAppendage.length;

        edits.forEach((edit) => {
            lintedText = lintedText.slice(0, edit.start + addedCharacters) + edit.edit + lintedText.slice(edit.end + addedCharacters);
            addedCharacters += edit.edit.length - (edit.end - edit.start);
        });
        lintedText = lintedText + `</div>`;

        this.hilayer.innerHTML = lintedText;
    }

    selectCurrentLine() {
        this.syncScrollPositions()
        
        setTimeout(() => {
            const cursorPosition = this.text.selectionStart;
            const lines = this.text.value.substr(0, cursorPosition).split('\n');
            const currentLineIndex = lines.length - 1;

            // remove active class from all line numbers
            this.margin.querySelectorAll('.line-number').forEach(lineNumber => {
                lineNumber.classList.remove('active-line-number');
            });

            // add active class to the current line number
            const currentLineNumber = this.margin.childNodes[currentLineIndex]?.childNodes[0];
            if (currentLineNumber) {
                currentLineNumber.classList.add('active-line-number');
            }
        }, 0);
    }

    refresh() {
        const fragment = document.createDocumentFragment();

        // add missing lines
        for (let i = this.margin.children.length; i < this.lines.length; i++) {
            const lineMargin = document.createElement('div');
            lineMargin.classList.add('line-margin');

            const lineNumber = document.createElement('div');
            lineNumber.classList.add('line-number');
            lineNumber.innerText = i + 1;

            lineMargin.appendChild(lineNumber);
            fragment.appendChild(lineMargin);
        }

        // remove extra lines
        while (this.margin.children.length > this.lines.length) {
            this.margin.removeChild(this.margin.lastChild);
        }

        // append new lines in a single operation
        this.margin.appendChild(fragment);
    }
}