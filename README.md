# bypassOS

**bypassOS** is a lightweight, fast, privacy-first Linux distribution built from the ground up to bypass censorship and protect user data. Designed to run smoothly even on older hardware, it strips away bloatware and tracking while keeping essential security, anti-censorship tools, and smooth desktop visuals ready out of the box.

> **⚠️ Work in Progress:** bypassOS is currently under active development. Features, configurations, and system components are subject to frequent changes.

## Key Features

- **Censorship Circumvention:** Built-in tools and pre-configured network routing to bypass restrictions and Deep Packet Inspection (DPI). Out of the box, bypassOS uses **[BlindDPI](https://github.com/bypassOS-dev/BlindDPI)** — a high-performance traffic obfuscation and DPI bypass tool written in Rust.
- **Zero Telemetry:** Completely free of tracking, analytics, or background data collection.
- **Lightweight & Fast:** Minimal footprint (~5 GB disk usage) and ultra-low memory consumption (~500 MB RAM at idle).
- **Pretty Out of the Box:** Powered by **Wayland** with smooth animations and classic **Wobbly Windows** physics enabled by default — proving that a lightweight, security-focused system doesn't have to look boring.

## System Requirements

| Component | Minimum | Recommended |
| :--- | :--- | :--- |
| **Architecture** | x86_64 | x86_64 |
| **RAM** | 512 MB | 1 GB+ |
| **Storage** | 7 GB | 10 GB+ |

## Getting Started

*Installation instructions, ISO downloads, and build scripts will be added here as the first public release approaches. The `.iso` image is currently in the works — stay tuned for the first release!*

### Quick Test (VM)

1. Download the latest `bypassOS.iso` from the [Releases](#) page.
2. Create a new virtual machine (QEMU, VirtualBox, or VMware) with at least **512 MB+ RAM** and **7 GB+** of disk space.
3. Boot from the ISO image.

## Roadmap

- [x] Core system base & minimal footprint optimization (~500 MB RAM)
- [x] Complete removal of telemetry and unnecessary services
- [x] Integration of [BlindDPI](https://github.com/bypassOS-dev/BlindDPI) for network-layer DPI circumvention
- [x] Wayland desktop environment with Wobbly Windows effects
- [ ] Pre-configured VPN/proxy suite & full-disk encryption options
- [ ] First public `.iso` release

## Contributing

Contributions, bug reports, and security audits are welcome. Feel free to open an issue or submit a pull request. Any help is greatly appreciated!
