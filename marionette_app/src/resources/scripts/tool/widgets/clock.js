class ClockWidget extends Widget {
    constructor(title, width, height) {
        super(title, width, height);
        this.clock = this.element.appendChild(document.createElement('div'));
        this.clock.classList.add('clock');
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