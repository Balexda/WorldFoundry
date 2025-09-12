# World Foundry

A high-performance, cross-platform fantasy map generator that imports maps from Azgaar's Fantasy Map Generator while providing superior native performance on Windows, iPad, and Android.

## 🎯 Project Goals

- **Native Performance**: Significantly faster than web-based solutions
- **Cross-Platform**: Single codebase supporting Windows, iPad, and Android
- **Azgaar Compatible**: Full import support for .map and JSON formats
- **Touch Optimized**: Intuitive interfaces designed for each platform
- **Offline Capable**: Full functionality without internet connection

## 🏗️ Architecture

World Foundry uses a shared Rust core engine with platform-specific UI implementations:

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────┬─────────────────┬─────────────────────────┤
│   Windows UI    │    iPad UI      │     Android UI          │
├─────────────────┼─────────────────┼─────────────────────────┤
│              Platform Abstraction Layer                    │
├─────────────────────────────────────────────────────────────┤
│                 Shared Core Engine (Rust)                  │
│  ┌─────────────┬─────────────────┬─────────────────────────┐ │
│  │   World     │    Rendering    │    Import/Export        │ │
│  │ Generation  │     Engine      │       System            │ │
│  └─────────────┴─────────────────┴─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ Technology Stack

- **Core Engine**: Rust with Skia for cross-platform 2D rendering
- **Windows**: WinUI 3 + .NET with Rust bindings
- **iPad**: SwiftUI with Rust static library
- **Android**: Jetpack Compose with JNI bindings

## 📋 Development Phases

- **Phase 1**: Core Engine (8-10 weeks)
- **Phase 2**: Platform Foundations (6-8 weeks)
- **Phase 3**: Feature Implementation (10-12 weeks)
- **Phase 4**: Polish and Testing (4-6 weeks)

**Total Timeline**: 28-36 weeks | **MVP**: 20 weeks

## 🚀 Getting Started

> **Note**: This project is in early development. Check back soon for build instructions and documentation.

## 📖 Documentation

- [Project Proposal](docs/Proposal.md) - Detailed technical specification and implementation plan

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines (coming soon).

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Azgaar's Fantasy Map Generator](https://azgaar.github.io/Fantasy-Map-Generator/) - Inspiration and import compatibility
- The fantasy mapping community for feedback and requirements

---

**World Foundry** - Forge your fantasy worlds with native performance.