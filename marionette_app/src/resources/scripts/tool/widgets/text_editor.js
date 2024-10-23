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
        this.hilayer.appendChild(document.createElement('div')).classList.add('line');
        this.text = this.createElementWithClass('textarea', 'text-area', this.container);

        this.current_linter_data = {
            "current_line": 0,
            "curr_text": "",
            "prev_text": "",

            "raw_curr_lines": [""]
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

    changedLines(strA, strB) {
        const linesA = strA.split('\n');
        const linesB = strB.split('\n');
        const changes = [];

        const maxLength = Math.max(linesA.length, linesB.length);

        for (let i = 0; i < maxLength; i++) {
            if (linesA[i] !== linesB[i]) {
                if (linesA[i] === undefined) {
                    changes.push({ line: i + 1, type: 'added', content: linesB[i] });
                } else if (linesB[i] === undefined) {
                    changes.push({ line: i + 1, type: 'removed', content: linesA[i] });
                } else {
                    changes.push({ line: i + 1, type: 'changed', content: linesB[i] });
                }
            }
        }

        return changes;
    }

    async onInput() {
        let prevText = this.current_linter_data["curr_text"];
        this.current_linter_data["prev_text"] = prevText;
        this.current_linter_data["curr_text"] = this.text.value;
        let textToLint = this.current_linter_data["curr_text"];
        this.current_linter_data["raw_curr_lines"] = textToLint.split('\n');

        this.syncScrollPositions();

        let changes = this.changedLines(prevText, textToLint);

        try {
            await this.updateText(changes);
        } catch (error) {
            console.error("Error processing linter data:", error);
        }

        this.refresh();
        this.selectCurrentLine();
    }

    async lintLine(lineContent) {
        var data = await window.internalRequest('lex', {"lexer":"lua", "text": lineContent}, false, true);
        data = JSON.parse(data["data"]);

        let edits = [];
        for (let i = 0; i < data.length; i++) {
            let tok = data[i];
            let slice = tok["slice"];
            let span = tok["span"];
            let start = span["start"];
            let end = span["end"];
            let token = tok["token"];

            let editContent = `<span class='${token.toLowerCase()}'>${slice}</span>`;
            edits.push({
                start: start,
                end: end,
                edit: editContent,
                token: token
            });
        }
        //edits.sort((a, b) => a.start - b.start);

        let addedCharacters = 0;
        edits.forEach((edit) => {
            lineContent = lineContent.slice(0, edit.start + addedCharacters) + edit.edit + lineContent.slice(edit.end + addedCharacters);
            addedCharacters += edit.edit.length - (edit.end - edit.start);
        });

        return lineContent;
    }

    updateText(data) {
        var lines = this.hilayer.getElementsByClassName('line');
        var lineRemovalCount = 0;

        data.forEach(async (line) => {
            if (line.type == 'added') {
                let lineElement = document.createElement('div');
                lineElement.classList.add('line');
                lineElement.innerHTML = await this.lintLine(line.content);
                this.hilayer.appendChild(lineElement);
            } else if (line.type == 'removed') {
                this.hilayer.removeChild(lines[line.line - (1 + lineRemovalCount)]);
                lineRemovalCount++;
            } else if (line.type == 'changed') {
                let lineElement = lines[line.line - 1];
                lineElement.innerHTML = await this.lintLine(line.content);
            }
        })
        this.refresh();
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
        const lines = this.hilayer.getElementsByClassName('line').length;

        // add missing lines
        for (let i = this.margin.children.length; i < lines; i++) {
            const lineMargin = document.createElement('div');
            lineMargin.classList.add('line-margin');

            const lineNumber = document.createElement('div');
            lineNumber.classList.add('line-number');
            lineNumber.innerText = i + 1;

            lineMargin.appendChild(lineNumber);
            fragment.appendChild(lineMargin);
        }

        // remove extra lines
        while (this.margin.children.length > lines) {
            this.margin.removeChild(this.margin.lastChild);
        }

        // append new lines in a single operation
        this.margin.appendChild(fragment);
    }
}