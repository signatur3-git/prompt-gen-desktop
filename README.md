# Random Prompt Generator - Desktop Application

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
- [Windows (x64)](https://github.com/[your-org]/prompt-gen-desktop/releases)
- [macOS (Intel/Apple Silicon)](https://github.com/[your-org]/prompt-gen-desktop/releases)
- [Linux (AppImage)](https://github.com/[your-org]/prompt-gen-desktop/releases)

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

- [Getting Started Guide](https://github.com/[your-org]/prompt-gen-spec/blob/main/docs/guides/getting-started.md)
- [Tutorial Series](https://github.com/[your-org]/prompt-gen-spec/blob/main/docs/guides/tutorial-series/)
- [Full Documentation](https://github.com/[your-org]/prompt-gen-spec/tree/main/docs)

## Related Projects

- [RPG Specification](https://github.com/[your-org]/prompt-gen-spec) - Format specification
- [RPG CLI](https://github.com/[your-org]/prompt-gen-cli) - Command-line tools (coming soon)

## Version

**Current:** v1.0.0 (External)  
**Internal:** v1.1.0 (Includes Dependencies & Rulebooks)

## License

MIT License - See LICENSE file

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- [Issue Tracker](https://github.com/[your-org]/prompt-gen-desktop/issues)
- [Discussions](https://github.com/[your-org]/prompt-gen-desktop/discussions)
- [Specification Repository](https://github.com/[your-org]/prompt-gen-spec)