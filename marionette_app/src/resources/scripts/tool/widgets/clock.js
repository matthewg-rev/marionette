class ClockWidget extends Widget {
    constructor(title, height, width) {
        super(title, height, width);
        this.clock = this.element.appendChild(document.createElement('div'));
        this.clock.id = 'clock';
        this.clock.innerHTML = '00:00:00';
        
        this.onExpand['clock'] = () => {
            this.clock.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
        }

        setInterval(() => {
            const date = new Date();
            this.clock.innerHTML = date.toLocaleTimeString();
        }, 1000);
    }
}