# Random Prompt Generator - Desktop Application

[![Version](https://img.shields.io/github/v/release/signatur3-git/prompt-gen-desktop?label=version&color=blue)](https://github.com/signatur3-git/prompt-gen-desktop/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](https://github.com/signatur3-git/prompt-gen-desktop/releases)
[![Downloads](https://img.shields.io/github/downloads/signatur3-git/prompt-gen-desktop/total?color=orange)](https://github.com/signatur3-git/prompt-gen-desktop/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/signatur3-git/prompt-gen-desktop/build.yml?branch=main&label=build)](https://github.com/signatur3-git/prompt-gen-desktop/actions)

Desktop authoring tool for creating and editing Random Prompt Generator (RPG) packages.

## Features

- 📝 **Visual Package Editor** - Create packages without writing YAML
- 🎲 **Live Preview** - See rendered prompts in real-time
- ✅ **Validation** - Real-time error detection with jump-to-error
- 📚 **Rulebooks** - Create prompt templates with weighted entry points
- 🔗 **Dependencies** - Reuse packages with semver version management
- 💾 **YAML Export** - Clean, readable package format

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform:
- [Windows (x64)](https://github.com/signatur3-git/prompt-gen-desktop/releases/latest)
- [macOS (Intel/Apple Silicon)](https://github.com/signatur3-git/prompt-gen-desktop/releases/latest)
- [Linux (AppImage)](https://github.com/signatur3-git/prompt-gen-desktop/releases/latest)

Or browse all releases: [Releases Page](https://github.com/signatur3-git/prompt-gen-desktop/releases)

### Build from Source

**Prerequisites:**
- Node.js 18+ and npm
- Rust 1.70+
- Platform-specific dependencies (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

**Build:**
```bash
# Install dependencies
npm install

# Development mode
npm run tauri dev

# Production build
npm run tauri build
```

## Usage

1. **Create Package** - File → New Package
2. **Add Entities** - Datatypes, Prompt Sections, Separators, Rules
3. **Preview** - See rendered output in Live Preview panel
4. **Validate** - Check for errors in Validation panel
5. **Save** - File → Save Package

## Documentation

- [Getting Started Guide](https://github.com/signatur3-git/prompt-gen-spec/blob/main/docs/guides/getting-started.md)
- [Tutorial Series](https://github.com/signatur3-git/prompt-gen-spec/blob/main/docs/guides/tutorial-series/)
- [Full Documentation](https://github.com/signatur3-git/prompt-gen-spec/tree/main/docs)
- [CLI Testing Commands](https://github.com/signatur3-git/prompt-gen-spec/blob/main/milestones/M10/M10_EXECUTION_GUIDE.md#step-23-test-cli-binary)

## Related Projects

- **[RPG Specification](https://github.com/signatur3-git/prompt-gen-spec)** - Format specification and documentation
- **RPG CLI** - Command-line tools (included in this release)
- **RPG Web** - Web application (coming in 2026)
- **RPG Marketplace** - Package registry (planned)

## Version

**Current Release:** v1.0.0-rc (Release Candidate)  
**Status:** Feature complete, testing phase  
**Next:** v1.0.0 (Final release)

**What's Included:**
- Desktop authoring tool (Tauri + Vue + Rust)
- CLI tools (rpg-cli.exe for validation and rendering)
- Full M1-M9 features (rendering, validation, rulebooks, dependencies)
- Cross-platform support (Windows, macOS, Linux)

## License

MIT License - See LICENSE file

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- [Issue Tracker](https://github.com/[your-org]/prompt-gen-desktop/issues)
- [Discussions](https://github.com/[your-org]/prompt-gen-desktop/discussions)
- [Specification Repository](https://github.com/[your-org]/prompt-gen-spec)