class Netstat {
    constructor(parentId) {
        if (!parentId) throw "Missing parameters";

        // Create DOM (compact: 2 lines)
        this.parent = document.getElementById(parentId);
        this.parent.innerHTML += `
        <div id="mod_netstat">
            <div id="mod_netstat_inner">
                <h1>Network<i id="mod_netstat_iname"></i></h1>
                <div id="mod_netstat_innercontainer">
                    <div>
                        <h2 id="net_local">Local: --</h2>
                        <h2 id="net_public">Public: --</h2>
                    </div>
                </div>
            </div>
        </div>`;

        // State
        this.offline = true;
        this.iface = null;
        this.internalIPv4 = null;
        this.ipinfo = { ip: null, geo: null };
        this._publicIpCache = { value: null, expiresAt: 0 };

        // Initial populate
        this.updateLocalIP();
        this.updatePublicIP(true);

        // Polling with adaptive throttling
        this._applyPerfMode = () => {
            const slowFactor = (window.__perf && window.__perf.slowFactor) ? window.__perf.slowFactor : 5;
            const isActive = window.__perf ? window.__perf.active : true;
            const localBase = 5000;   // 5s
            const publicBase = 60000; // 60s
            const localMs = isActive ? localBase : localBase * slowFactor;
            const publicMs = isActive ? publicBase : publicBase * slowFactor;

            if (this._localTimer) clearInterval(this._localTimer);
            if (this._publicTimer) clearInterval(this._publicTimer);
            this._localTimer = setInterval(() => this.updateLocalIP(), localMs);
            this._publicTimer = setInterval(() => this.updatePublicIP(), publicMs);
        };
        this._applyPerfMode();
        window.addEventListener('perf-mode-change', this._applyPerfMode);
    }

    async updateLocalIP() {
        const os = require('os').platform();
        const cp = require('child_process');
        const exec = (cmd) => new Promise(resolve => {
            cp.exec(cmd, { timeout: 2000 }, (e, so, se) => {
                const out = (so && so.trim()) || '';
                resolve(out);
            });
        });

        let ip = '';
        try {
            if (os === 'darwin') {
                ip = await exec('ipconfig getifaddr en0');
                if (!ip) ip = await exec('ipconfig getifaddr en1');
            } else if (os === 'linux') {
                ip = await exec("sh -lc \"hostname -I | awk '{print $1}'\"");
            } else if (os === 'win32') {
                const out = await exec('ipconfig');
                const m = out.match(/IPv4 Address[^:]*:\\s*([0-9.]+)/i);
                ip = m ? m[1] : '';
            }
        } catch(_) { /* ignore */ }

        if (!ip) ip = '--';
        const localNode = document.getElementById('net_local');
        if (localNode) localNode.textContent = `Local: ${ip}`;
        this.internalIPv4 = ip;
        this.offline = (!ip || ip === '--' || ip.startsWith('127.'));

        // Try to resolve iface for other modules
        try {
            if (window.si && ip && ip !== '--') {
                const nics = await window.si.networkInterfaces();
                const found = nics.find(n => n.ip4 === ip && n.operstate === 'up');
                this.iface = found ? found.iface : (nics.find(n => n.ip4 === ip) || {}).iface || null;
                const iname = document.getElementById('mod_netstat_iname');
                if (iname) iname.innerText = this.iface ? `Interface: ${this.iface}` : 'Interface: (offline)';
            } else {
                this.iface = null;
            }
        } catch(_) {
            this.iface = null;
        }
    }

    async updatePublicIP(force = false) {
        const now = Date.now();
        const ttlMs = 60 * 1000; // 60s cache
        if (!force && this._publicIpCache.value && this._publicIpCache.expiresAt > now) {
            const node = document.getElementById('net_public');
            if (node) node.textContent = `Public: ${this._publicIpCache.value}`;
            this.ipinfo.ip = this._publicIpCache.value;
            return;
        }

        const cp = require('child_process');
        const https = require('https');
        const exec = (cmd) => new Promise(resolve => {
            cp.exec(cmd, { timeout: 2500 }, (e, so, se) => resolve((so || '').trim()));
        });
        const isIP = (s) => /^\d+\.\d+\.\d+\.\d+$/.test(s);

        let pip = '';
        try {
            pip = await exec('curl -s ifconfig.me');
            if (!isIP(pip)) pip = await exec('curl -s api.ipify.org');
        } catch(_) { /* ignore */ }

        if (!isIP(pip)) {
            // Fallback to https if curl unavailable
            pip = await new Promise(resolve => {
                try {
                    https.get('https://api.ipify.org', res => {
                        let data = '';
                        res.on('data', c => data += c);
                        res.on('end', () => resolve((data || '').trim()))
                    }).on('error', () => resolve(''));
                } catch(_) { resolve(''); }
            });
        }

        if (!isIP(pip)) pip = '--';
        const pubNode = document.getElementById('net_public');
        if (pubNode) pubNode.textContent = `Public: ${pip}`;
        if (isIP(pip)) {
            this._publicIpCache = { value: pip, expiresAt: now + ttlMs };
            this.ipinfo.ip = pip;
        }
    }
}

module.exports = {
    Netstat
};

