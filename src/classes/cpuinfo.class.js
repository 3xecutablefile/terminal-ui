class Cpuinfo {
    constructor(parentId) {
        if (!parentId) throw "Missing parameters";

        // Create initial DOM
        this.parent = document.getElementById(parentId);
        this.parent.innerHTML += `<div id="mod_cpuinfo">
        </div>`;
        this.container = document.getElementById("mod_cpuinfo");

        // Init Smoothie
        let TimeSeries = require("smoothie").TimeSeries;
        let SmoothieChart = require("smoothie").SmoothieChart;

        this.series = [];
        this.charts = [];
        window.si.cpu().then(data => {
            let divide = Math.floor(data.cores/2);
            this.divide = divide;

            let cpuName = data.manufacturer+data.brand;
            cpuName = cpuName.substr(0, 30);
            cpuName.substr(0, Math.min(cpuName.length, cpuName.lastIndexOf(" ")));

            let innercontainer = document.createElement("div");
            innercontainer.setAttribute("id", "mod_cpuinfo_innercontainer");
            innercontainer.innerHTML = `<h1>CPU USAGE<i>${cpuName}</i></h1>
                <div>
                    <h1># <em>1</em> - <em>${divide}</em><br>
                    <i id="mod_cpuinfo_usagecounter0">Avg. --%</i></h1>
                    <canvas id="mod_cpuinfo_canvas_0" height="60"></canvas>
                </div>
                <div>
                    <h1># <em>${divide+1}</em> - <em>${data.cores}</em><br>
                    <i id="mod_cpuinfo_usagecounter1">Avg. --%</i></h1>
                    <canvas id="mod_cpuinfo_canvas_1" height="60"></canvas>
                </div>
                <div>
                    <div>
                        <h1>${(process.platform === "win32") ? "CORES" : "TEMP"}<br>
                        <i id="mod_cpuinfo_temp">${(process.platform === "win32") ? data.cores : "--°C"}</i></h1>
                    </div>
                    <div>
                        <h1>SPD<br>
                        <i id="mod_cpuinfo_speed_min">--GHz</i></h1>
                    </div>
                    <div>
                        <h1>MAX<br>
                        <i id="mod_cpuinfo_speed_max">--GHz</i></h1>
                    </div>
                    <div>
                        <h1>TASKS<br>
                        <i id="mod_cpuinfo_tasks">---</i></h1>
                    </div>
                </div>`;
            this.container.append(innercontainer);

            for (var i = 0; i < 2; i++) {
                this.charts.push(new SmoothieChart({
                    limitFPS: 30,
                    responsive: true,
                    millisPerPixel: 50,
                    interpolation: 'bezier',
                    grid:{
                        fillStyle:'transparent',
                        strokeStyle:'transparent',
                        verticalSections:0,
                        borderVisible:false
                    },
                    labels:{
                        disabled: true
                    },
                    yRangeFunction: () => {
                        return {min:0,max:100};
                    }
                }));
            }

            for (var i = 0; i < data.cores; i++) {
                // Create TimeSeries
                this.series.push(new TimeSeries());

                let serie = this.series[i];
                let options = {
                    lineWidth: 1.7,
                    strokeStyle: `rgb(${window.theme.r},${window.theme.g},${window.theme.b})`
                };

                if (i < divide) {
                    this.charts[0].addTimeSeries(serie, options);
                } else {
                    this.charts[1].addTimeSeries(serie, options);
                }
            }

            for (var i = 0; i < 2; i++) {
                this.charts[i].streamTo(document.getElementById(`mod_cpuinfo_canvas_${i}`), 500);
            }

            // Init updater
            this.updatingCPUload = false;
            this.updateCPUload();
            if (process.platform !== "win32") {this.updateCPUtemp();}
            this.updatingCPUspeed = false;
            this.updateCPUspeed();
            this.updatingCPUtasks = false;
            this.updateCPUtasks();
            // Adaptive refresh intervals (fast when focused/visible, slow when idle/hidden)
            this._applyPerfMode = () => {
                const fast = (m) => window.__perf ? window.__perf.getFast(m) : 1000*m;
                const slow = (m) => window.__perf ? window.__perf.getSlow(m) : 5000*m;
                const use = (fm, sm) => (window.__perf && window.__perf.active ? fm : sm);

                // Update Smoothie FPS to reduce GPU/CPU when idle
                try {
                    const active = (window.__perf && window.__perf.active);
                    const targetFps = active ? 30 : 12;
                    const mpp = active ? 50 : 90;
                    const mode = active ? 'bezier' : 'linear';
                    for (let i = 0; i < this.charts.length; i++) {
                        if (this.charts[i] && this.charts[i].options) {
                            this.charts[i].options.limitFPS = targetFps;
                            this.charts[i].options.millisPerPixel = mpp;
                            this.charts[i].options.interpolation = mode;
                        }
                    }
                } catch(_) {}

                // Clear existing timers
                if (this.loadUpdater) clearInterval(this.loadUpdater);
                if (this.tempUpdater) clearInterval(this.tempUpdater);
                if (this.speedUpdater) clearInterval(this.speedUpdater);
                if (this.tasksUpdater) clearInterval(this.tasksUpdater);

                // Set new timers based on perf mode
                this.loadUpdater = setInterval(() => this.updateCPUload(), use(fast(0.5), slow(1.5)));
                if (process.platform !== "win32") {
                    this.tempUpdater = setInterval(() => this.updateCPUtemp(), use(fast(2.0), slow(5.0)));
                }
                this.speedUpdater = setInterval(() => this.updateCPUspeed(), use(fast(1.0), slow(2.0)));
                this.tasksUpdater = setInterval(() => this.updateCPUtasks(), use(fast(5.0), slow(10.0)));
            };
            this._applyPerfMode();
            window.addEventListener('perf-mode-change', this._applyPerfMode);
        });
    }
    updateCPUload() {
        if (this.updatingCPUload) return;
        this.updatingCPUload = true;
        window.si.currentLoad().then(data => {
            let average = [[], []];

            if (!data.cpus) return; // Prevent memleak in rare case where systeminformation takes extra time to retrieve CPU info (see github issue #216)

            data.cpus.forEach((e, i) => {
                this.series[i].append(new Date().getTime(), e.load);

                if (i < this.divide) {
                    average[0].push(e.load);
                } else {
                    average[1].push(e.load);
                }
            });
            average.forEach((stats, i) => {
                average[i] = Math.round(stats.reduce((a, b) => a + b, 0)/stats.length);

                try {
                    document.getElementById(`mod_cpuinfo_usagecounter${i}`).innerText = `Avg. ${average[i]}%`;
                } catch(e) {
                    // Fail silently, DOM element is probably getting refreshed (new theme, etc)
                }
            });
            this.updatingCPUload = false;
        });
    }
    updateCPUtemp() {
        window.si.cpuTemperature().then(data => {
            try {
                document.getElementById("mod_cpuinfo_temp").innerText = `${data.max}°C`;
            } catch(e) {
                // See above notice
            }
        });
    }
    updateCPUspeed() {
        if (this.updatingCPUspeed) return;
        this.updatingCPUspeed = true
        window.si.cpu().then(data => {
            try {
                document.getElementById("mod_cpuinfo_speed_min").innerText = `${data.speed}GHz`;
                document.getElementById("mod_cpuinfo_speed_max").innerText = `${data.speedMax}GHz`;
            } catch(e) {
                // See above notice
            }
            this.updatingCPUspeed = false;
        });
    }
    updateCPUtasks() {
        if (this.updatingCPUtasks) return;
        this.updatingCPUtasks = true;
        window.si.processes().then(data => {
            try {
                document.getElementById("mod_cpuinfo_tasks").innerText = `${data.all}`;
            } catch(e) {
                // See above notice
            }
            this.updatingCPUtasks = false;
        });
    }
}

module.exports = {
    Cpuinfo
};
