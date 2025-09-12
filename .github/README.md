# GitHub Workflows and Templates

This directory contains GitHub Actions workflows, issue templates, and pull request templates for the World Foundry project.

## 🔄 Workflows

### Core Workflows

#### `ci.yml` - Continuous Integration
**Triggers:** Push to main/develop, Pull Requests
**Purpose:** Comprehensive testing and quality assurance

**Jobs:**
- **Test Suite** - Cross-platform testing (Ubuntu, Windows, macOS) with Rust stable/beta
- **Security Audit** - Dependency vulnerability scanning with `cargo audit`
- **Code Coverage** - Coverage reporting with `cargo-llvm-cov` and Codecov integration
- **Build Artifacts** - Release binary builds for all target platforms
- **Benchmarks** - Performance testing on pull requests
- **Documentation** - API documentation generation and deployment

**Features:**
- ✅ Cross-platform compatibility testing
- ✅ Rust formatting and Clippy linting
- ✅ System dependency installation
- ✅ Comprehensive test execution
- ✅ CLI tool validation
- ✅ Example execution verification

#### `release.yml` - Automated Releases
**Triggers:** Git tags (`v*`), Manual workflow dispatch
**Purpose:** Automated release creation and distribution

**Jobs:**
- **Create Release** - GitHub release creation with changelog
- **Build Release** - Multi-platform binary compilation and upload
- **Publish Crate** - Automated crates.io publishing
- **Update Homebrew** - Homebrew formula updates (when available)

**Artifacts:**
- `world-foundry-linux` - Linux x86_64 binary
- `world-foundry-windows.exe` - Windows x86_64 binary
- `world-foundry-macos` - macOS Intel binary
- `world-foundry-macos-arm64` - macOS Apple Silicon binary

#### `dependencies.yml` - Dependency Management
**Triggers:** Weekly schedule (Mondays 9 AM UTC), Manual dispatch
**Purpose:** Automated dependency updates and security monitoring

**Jobs:**
- **Update Rust Dependencies** - Automated `cargo upgrade` with PR creation
- **Security Audit** - Weekly vulnerability scanning
- **License Check** - License compliance verification

**Features:**
- 🔄 Automated dependency updates
- 🔒 Security vulnerability detection
- ⚖️ License compliance monitoring
- 📝 Automated pull request creation

#### `quality.yml` - Code Quality Assurance
**Triggers:** Push to main/develop, Pull Requests, Weekly schedule
**Purpose:** Code quality, formatting, and documentation standards

**Jobs:**
- **Formatting** - Rust and Markdown formatting verification
- **Linting** - Advanced Clippy linting with pedantic rules
- **Complexity** - Code metrics analysis with `tokei`
- **Documentation** - Documentation coverage and link checking
- **Performance** - Performance impact analysis on PRs
- **Spell Check** - Documentation spell checking with `typos`

### Platform-Specific Workflows

#### `platform-builds.yml` - Platform Development
**Triggers:** Changes to platform-specific code
**Purpose:** Platform-specific build validation (currently disabled)

**Future Jobs:**
- **Windows Platform** - WinUI 3 application builds
- **iOS Platform** - SwiftUI application builds with Xcode
- **Android Platform** - Jetpack Compose builds with Gradle
- **Web Platform** - WASM compilation and web app builds
- **Integration Test** - Cross-platform compatibility verification

*Note: These jobs are currently disabled (`if: false`) until platform implementations exist.*

## 📋 Issue Templates

### `bug_report.yml` - Bug Reports
Comprehensive bug reporting with:
- Platform and version information
- Detailed reproduction steps
- Environment details and error logs
- Severity assessment
- Sample file information

### `feature_request.yml` - Feature Requests
Structured feature requests including:
- Feature categorization and priority
- Problem statement and proposed solution
- Use cases and acceptance criteria
- Technical considerations
- Contribution opportunities

### `platform_support.yml` - Platform Support
Platform-specific requests covering:
- New platform support requests
- Platform-specific bugs and optimizations
- Technical challenges and requirements
- User base impact assessment
- Testing and development contribution

### `config.yml` - Issue Template Configuration
- Disables blank issues
- Provides links to discussions, documentation, and guides
- Directs users to appropriate resources

## 📝 Pull Request Template

Comprehensive PR template with:
- Change type classification
- Testing requirements checklist
- Platform impact assessment
- Security and performance considerations
- Documentation update tracking
- Reviewer guidelines

## 🔧 Workflow Configuration

### Environment Variables
- `CARGO_TERM_COLOR=always` - Colored Cargo output
- `RUST_BACKTRACE=1` - Detailed error backtraces

### Caching Strategy
- **Rust Cache** - `Swatinem/rust-cache@v2` for dependency caching
- **Workspace-aware** - Optimized for `core/` workspace structure
- **Cross-platform** - Separate caches per OS and architecture

### System Dependencies
Automated installation of required system libraries:
- **Ubuntu**: Build tools, graphics libraries (OpenGL, EGL, XCB)
- **macOS**: Platform-specific dependencies as needed
- **Windows**: Platform-specific dependencies as needed

### Security Considerations
- **Token Usage** - Minimal required permissions
- **Audit Integration** - Regular security scanning
- **License Compliance** - Automated license checking
- **Dependency Updates** - Proactive vulnerability management

## 🚀 Getting Started

### For Contributors
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure all CI checks pass locally:
   ```bash
   cd core
   cargo fmt --all -- --check
   cargo clippy --all-features -- -D warnings
   cargo test --all-features
   cargo build --release
   ```
5. Submit a pull request using the provided template

### For Maintainers
1. **Release Process**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
   This triggers the automated release workflow.

2. **Manual Dependency Updates**:
   - Go to Actions → Dependencies → Run workflow
   - Review and merge the automated PR

3. **Security Monitoring**:
   - Weekly security audits run automatically
   - Check Actions tab for any security alerts

## 📊 Workflow Status

Current workflow status and coverage:
- ✅ **CI Pipeline** - Fully implemented and tested
- ✅ **Release Automation** - Ready for v1.0.0 release
- ✅ **Quality Assurance** - Comprehensive code quality checks
- ✅ **Security Monitoring** - Automated vulnerability scanning
- 🚧 **Platform Builds** - Templates ready, awaiting platform implementations
- ✅ **Issue Management** - Complete template system
- ✅ **Documentation** - Automated API doc deployment

## 🔗 Related Documentation

- [Project Architecture](../docs/architecture/ARCHITECTURE.md)
- [Development Proposal](../docs/Proposal.md)
- [Project Status](../docs/PROJECT_STATUS.md)
- [Core Engine Documentation](../core/README.md)