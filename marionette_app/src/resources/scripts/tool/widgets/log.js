function hook(object, name, callback) {
    let original_fn = null;
    (function(original) {
        original_fn = original;
        object[name] = function() {
            var ret = original.apply(this, arguments);
            ret = callback(ret, arguments);
            return ret;
        }
    })(object[name]);
    return original_fn;
}

function restore(object, name, original) {
    object[name] = original;
}

class LogWidget extends Widget {
    constructor(title, width, height) {
        super(title, width, height);

        this.bottomBar = document.createElement('div');
        this.bottomBar.classList.add('bottom-bar');
        this.element.appendChild(this.bottomBar);

        this.container = document.createElement('table');
        this.container.classList.add('log-container');
        this.container.style.width = '100%'; // Set the table width to 100%
        this.container.style.height = 'calc(100% - 42px)'; // Adjust height as before
        this.element.appendChild(this.container);

        this.original_received = null;
        this.original_requested = null;
        this.original_error = null;

        this.cleanup = () => {
            if (this.original_received) {
                restore(window, 'received', this.original_received);
                this.original_received = null;
            }
            if (this.original_requested) {
                restore(window, 'requested', this.original_requested);
                this.original_requested = null;
            }
            if (this.original_error) {
                restore(window, 'error', this.original_error);
                this.original_error = null;
            }
        }

        this.onExpand['log'] = () => {
            this.bottomBar.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            this.container.style.height = this.flags.expanded ? 'calc(100% - 42px)' : '0';

            if (this.flags.expanded) {

                this.original_received = hook(window, 'received', (ret, args) => {
                    let content = JSON.parse(args[0]);
                    this.createLog("RECV", content.status, content.data);
                    return ret;
                });

                this.original_requested = hook(window, 'requested', (ret, args) => {
                    this.createLog("CREQ", args[0].method, args[0].data);
                    return ret;
                });

                this.original_error = hook(window, 'error', (ret, args) => {
                    // TODO: behavior is undefined currently.
                    this.createLog("CERR", "error", args[0]);
                    return ret;
                });
            } else {
                restore(window, 'received', this.original_received);
                restore(window, 'requested', this.original_requested);
                restore(window, 'error', this.original_error);
            }
        }
    }

    logColor(detail) {
        let color = 'log-default';
        if (detail == 'error') {
            color = 'log-error';
        } else if (detail == 'ok') {
            color = 'log-info';
        } else if (detail == 'info') {
            color = 'log-info';
        }
        return color;
    }

    createLog(type, detail, message) {
        message = message == "" ? "(empty)" : message;
        let logObject = document.createElement('tr');
        logObject.classList.add('log');

        let timeObject = document.createElement('td');
        timeObject.classList.add('log-time');
        timeObject.textContent = new Date().toLocaleTimeString();

        let typeObject = document.createElement('td');
        typeObject.classList.add('log-type');
        typeObject.textContent = type;

        let detailObject = document.createElement('td');
        detailObject.classList.add('log-detail');
        detailObject.textContent = detail;

        let messageObject = document.createElement('td');
        messageObject.classList.add('log-message');
        messageObject.textContent = message;

        let color = this.logColor(detail);
        typeObject.classList.add(color);
        detailObject.classList.add(color);
        messageObject.classList.add(color);

        logObject.appendChild(timeObject);
        logObject.appendChild(typeObject);
        logObject.appendChild(detailObject);
        logObject.appendChild(messageObject);
        this.container.appendChild(logObject);

        this.container.scrollTop = this.container.scrollHeight;
    }
}