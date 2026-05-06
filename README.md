# JUX
### *Julas Untracked Linux*

> **Your computer, yours again.**
> Carry everything. Leave nothing.

---

## What is JUX?

JUX is a minimal, web-native Linux environment built on a single conviction: your computer should belong to you — completely, unconditionally, and without compromise.

It is not a distribution. It is not a desktop environment in the traditional sense. JUX is a philosophy made bootable.

The entire system fits on a USB drive or a MicroSD card. It boots into a read-only image, opens a browser, and becomes your complete personal computing environment — at home, at work, on a Raspberry Pi, on a corporate network, or from a hotel computer on the other side of the world.

Your data lives where you decide. Your system leaves no trace where it runs. When you leave, nothing stays behind.

---

## Why JUX Exists

Somewhere along the way, the personal computer stopped being personal.

Operating systems became platforms for telemetry and metadata collective. Applications became subscriptions. Your files became someone else's cloud. Your habits became someone else's dataset. The machine on your desk runs your software, but serves someone else's interests.

JUX exists because that is unacceptable.

Not out of paranoia. Out of principle.

---

## How It Works

JUX runs entirely inside a browser. There is no traditional desktop environment, no display server stack, no application installation process. The system boots a minimal Linux base — kernel, codecs, essential drivers, and a lightweight HTTP server — and then gets out of the way while the browser takes over.

Everything you see and use is a web application served locally:

- A **Markdown editor** for writing
- A **Spreadsheet** for calculation — formulas, no macros, no bloat
- An **Image viewer** supporting all common formats
- A **Video player** with hardware-accelerated decoding
- An **Audio player** for your local music library
- A **Terminal** running your actual shell — bash, zsh, or fish — with full theme support
- A **Code editor** via code-server, the full VS Code experience in a tab
- A **File manager** for navigating and organizing your data

These are not cloud apps. They are not calling home. They run on your hardware, serving only you.

---

## Your Data, Your Rules

JUX separates the system from the data completely and permanently.

```
SYSTEM (read-only)          YOUR DATA (yours alone)
────────────────────        ────────────────────────
USB drive / MicroSD         External SSD, local disk,
Immutable after boot        or network storage (NFS)
Replaceable, shareable      Encrypted, portable,
Zero personal data          irreplaceable
```

The system image is generic. It knows nothing about you. Your `/home` directory — your files, your configurations, your history — lives on a separate volume that you control. Encrypted with your passphrase. Mounted only after you authenticate. Never touched by the system image itself.

Lose the USB drive? Flash a new one. The image is public. Your data is safe elsewhere, because it was never on the drive to begin with.

---

## Where JUX Runs

**Personal** — Boot from USB or MicroSD on any x86_64 machine or Raspberry Pi 5. Your data on an external SSD or a second card. Carry your entire computing environment in your pocket.

**Home Server** — Run JUX on a Raspberry Pi 5 with 8GB RAM. Serve your personal environment to any browser in your home — or anywhere in the world via an encrypted tunnel. Always on, consuming less power than a light bulb.

**Enterprise / LAN** — Deploy via PXE/LTSP. Terminals boot the JUX image from the network into RAM. User data mounts from centralized NFS/iSCSI storage. No data on the endpoints. No OS to maintain on individual machines. One image update propagates everywhere.

**Remote / VDI** — Access your full JUX environment from any browser, anywhere. A hotel computer, a cybercafé, a borrowed Chromebook, a phone. Two-factor authentication. Zero persistence on the client device. Full session on your server, rendered locally in the browser.

---

## Design Principles

**1. The browser is the platform.**
If it cannot run in a browser, it does not belong in JUX. This is not a limitation — it is the architecture.

**2. Read-only by default.**
The system image is never written to during operation. Power loss cannot corrupt it. Updates are image replacements, not package installations.

**3. No telemetry. Ever.**
JUX does not phone home. It has no home to phone. There are no analytics, no crash reports, no usage metrics. What you do on your machine is your business alone.

**4. Separation of system and data.**
The OS and your personal data are physically separate at all times. This is not a configuration option. It is the fundamental design.

**5. Minimal by conviction, not by limitation.**
Every component in JUX earns its place. If the essential function can be achieved without a dependency, the dependency does not exist. Small is not a constraint — it is the goal.

**6. Hardware you already own.**
JUX is designed to run well on hardware from 10 to 15 years ago. Planned obsolescence is not a business model we participate in.

---

## The Stack

```
Hardware
└── Kernel (minimal modules: USB, net, audio, video, DRM)
    └── musl libc + BusyBox
        └── s6 (init/supervision)
            └── Wayland compositor (cage — single app, fullscreen)
                └── Chromium / compatible browser (kiosk mode)
                    └── JUX Shell (SPA — the desktop itself)
                        ├── App: Markdown Editor
                        ├── App: Spreadsheet
                        ├── App: Image Viewer
                        ├── App: Video Player
                        ├── App: Audio Player
                        ├── App: Terminal (xterm.js + WebSocket)
                        ├── App: File Manager
                        └── App: Code Editor (code-server)
```

Local daemon (single binary, Go or Rust):
- File system API (read/write `/home`)
- Media bridge (ffmpeg — format transcoding for browser)
- Shell bridge (WebSocket → bash/zsh/fish)
- System metrics (CPU, RAM, network → sidebar widgets)
- Authentication and session management

---

## Project Status

JUX is currently in the **concept and architecture phase**.

This document is the beginning. The repository will grow to include:

- [ ] Architecture specification
- [ ] Boot image build system (based on Buildroot or Yocto)
- [ ] Local daemon (API server)
- [ ] JUX Shell (the web desktop SPA)
- [ ] Core web applications
- [ ] PXE/LTSP deployment guide
- [ ] Raspberry Pi 5 image
- [ ] Enterprise deployment documentation

---

## Contributing

JUX is open to collaborators who share the conviction that personal computing should be personal.

If you believe that your computer should serve you — and only you — this is your project too.

More details on contribution guidelines coming as the architecture solidifies.

---

## The Name

**JUX** — *Julas Untracked Linux*

Named after a friend. Built on trust. The same trust this system places in you: that you know what to do with a computer that is truly yours.

---

## License

To be defined. The intent is fully open source — free to use, inspect, modify, and distribute. Details forthcoming.

---

*"Your computer, yours again. Carry everything. Leave nothing."*
