class HackerTools {
    constructor(parentId) {
        if (!parentId) throw "Missing parameters";

        const os = require("os").platform();
        const cp = require("child_process");

        this.parent = document.getElementById(parentId);
        this.parent.innerHTML += `
        <div id="mod_hackertools">
            <div id="mod_hackertools_inner">
                <h1>TOOLS QUICK-LAUNCH<i>NMAP • SQLMAP • HYDRA • MASSCAN • GOBUSTER • NIKTO • JOHN</i></h1>
                <div class="tools">
                    <button id="btn_nmap" class="tool">nmap</button>
                    <button id="btn_sqlmap" class="tool">sqlmap</button>
                    <button id="btn_hydra" class="tool">hydra</button>
                    <button id="btn_masscan" class="tool">masscan</button>
                    <button id="btn_gobuster" class="tool">gobuster</button>
                    <button id="btn_nikto" class="tool">nikto</button>
                    <button id="btn_john" class="tool">john</button>
                </div>
                <div class="status">
                    <span id="tool_nmap" class="probe">nmap: <i>checking…</i></span>
                    <span id="tool_sqlmap" class="probe">sqlmap: <i>checking…</i></span>
                    <span id="tool_hydra" class="probe">hydra: <i>checking…</i></span>
                    <span id="tool_masscan" class="probe">masscan: <i>checking…</i></span>
                    <span id="tool_gobuster" class="probe">gobuster: <i>checking…</i></span>
                    <span id="tool_nikto" class="probe">nikto: <i>checking…</i></span>
                    <span id="tool_john" class="probe">john: <i>checking…</i></span>
                </div>
                <div class="customTools" id="customTools" style="display:none">
                    <div class="sectionTitle">Custom Scripts</div>
                    <div class="tools" id="customToolsButtons"></div>
                </div>
            </div>
        </div>`;

        // Wire buttons
        document.getElementById("btn_nmap").onclick = () => this._promptAndRun(
            "Run nmap",
            "Target (host or CIDR)",
            val => `nmap -sV -T4 --top-ports 100 ${val}`
        );
        document.getElementById("btn_sqlmap").onclick = () => this._promptAndRun(
            "Run sqlmap",
            "Target URL",
            val => `sqlmap -u "${val}" --batch`
        );
        document.getElementById("btn_hydra").onclick = () => this._promptAndRun(
            "Run hydra",
            "host service (ex: 10.0.0.5 ssh)",
            val => `hydra -L users.txt -P pass.txt ${val}`
        );
        document.getElementById("btn_masscan").onclick = () => this._promptAndRun(
            "Run masscan",
            "Target (host or CIDR)",
            val => `masscan -p1-65535 --rate 1000 ${val}`
        );
        document.getElementById("btn_gobuster").onclick = () => this._promptAndRun(
            "Run gobuster dir",
            "Target URL",
            val => `gobuster dir -u "${val}" -w /usr/share/wordlists/dirb/common.txt -q`
        );
        document.getElementById("btn_nikto").onclick = () => this._promptAndRun(
            "Run nikto",
            "Target host or URL",
            val => `nikto -h "${val}"`
        );
        document.getElementById("btn_john").onclick = () => this._promptAndRun(
            "Run john",
            "Path to hash file",
            val => `john "${val}"`
        );

        // Probe availability and reflect in UI
        const whichCmd = (bin) => new Promise(resolve => {
            let cmd;
            if (os === 'win32') cmd = `where ${bin}`; else cmd = `command -v ${bin}`;
            cp.exec(cmd, { timeout: 1500 }, (e, so) => resolve(Boolean(!e && so && so.trim().length)));
        });

        const setProbe = (id, ok) => {
            const el = document.getElementById(id);
            if (!el) return;
            el.innerHTML = el.innerHTML.replace(/<i>.*<\/i>/, ok ? '<i>available</i>' : '<i>missing</i>');
            el.setAttribute('data-ok', ok ? '1' : '0');
        };

        whichCmd('nmap').then(ok => setProbe('tool_nmap', ok));
        whichCmd('sqlmap').then(ok => setProbe('tool_sqlmap', ok));
        whichCmd('hydra').then(ok => setProbe('tool_hydra', ok));
        whichCmd('masscan').then(ok => setProbe('tool_masscan', ok));
        whichCmd('gobuster').then(ok => setProbe('tool_gobuster', ok));
        whichCmd('nikto').then(ok => setProbe('tool_nikto', ok));
        whichCmd('john').then(ok => setProbe('tool_john', ok));

        // Load custom tools from config file
        try {
            const fs = require('fs');
            const path = require('path');
            const homedir = require('os').homedir();
            const cfgPath = process.env.EDEX_TOOLS_CONFIG || path.join(homedir, '.edex-tools.json');
            if (fs.existsSync(cfgPath)) {
                const raw = fs.readFileSync(cfgPath, 'utf8');
                const parsed = JSON.parse(raw);
                const list = Array.isArray(parsed) ? parsed : Array.isArray(parsed.scripts) ? parsed.scripts : [];
                if (list.length) {
                    const container = document.getElementById('customTools');
                    const btnWrap = document.getElementById('customToolsButtons');
                    container.style.display = '';
                    list.forEach((entry, idx) => {
                        if (!entry || !entry.label || !entry.command) return;
                        const id = `btn_custom_${idx}`;
                        const btn = document.createElement('button');
                        btn.className = 'tool';
                        btn.id = id;
                        btn.textContent = String(entry.label);
                        btnWrap.appendChild(btn);
                        const placeholder = entry.placeholder || 'Input (optional)';
                        btn.onclick = () => this._promptAndRun(`Run ${entry.label}`, placeholder, (val) => {
                            try {
                                return String(entry.command).replace(/\{input\}/g, val);
                            } catch (_) {
                                return String(entry.command);
                            }
                        });
                    });
                }
            }
        } catch (e) {
            // ignore malformed config
        }
    }

    _promptAndRun(title, placeholder, buildCmd) {
        // Detach keyboard to allow typing in modal inputs
        if (window.keyboard) window.keyboard.detach();
        let removed = false;
        new Modal({
            type: 'custom',
            title,
            html: `
                <div class="toolPrompt">
                    <input autofocus type="text" id="toolPromptInput" placeholder="${placeholder}" />
                    <div class="hint">Press Enter to run in the current terminal</div>
                </div>
            `
        }, () => {
            removed = true;
            if (window.keyboard) window.keyboard.attach();
            if (window.term && window.term[window.currentTerm]) window.term[window.currentTerm].term.focus();
        });

        const input = document.getElementById('toolPromptInput');
        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const val = input.value.trim();
                if (!val) return;
                const cmd = buildCmd(val);
                try {
                    window.term[window.currentTerm].writelr(cmd);
                } catch(_) {}
                // Auto-close modal
                if (!removed) {
                    const btn = document.querySelector('.modal button.close');
                    if (btn && typeof btn.click === 'function') {
                        btn.click();
                    } else {
                        // Fallback: reattach keyboard + refocus
                        if (window.keyboard) window.keyboard.attach();
                        if (window.term && window.term[window.currentTerm]) window.term[window.currentTerm].term.focus();
                    }
                }
            }
        });
    }
}

module.exports = {
    HackerTools
};
