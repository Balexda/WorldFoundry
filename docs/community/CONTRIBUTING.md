# Contributing to World Foundry

Welcome to the World Foundry community! We're excited to have you contribute to this cross-platform fantasy map generator. This guide will help you get started with contributing to the project.

## 🌟 Ways to Contribute

### Code Contributions
- **Core Engine Development** (Rust) - Improve the world generation algorithms
- **Platform Development** - Enhance Windows (WinUI 3), iOS (SwiftUI), or Android (Jetpack Compose) implementations
- **Bug Fixes** - Help resolve issues and improve stability
- **Performance Optimization** - Make the engine faster and more efficient
- **New Features** - Implement requested features from the roadmap

### Documentation Contributions
- **User Guides** - Help users understand how to use World Foundry
- **Developer Documentation** - Improve technical documentation for contributors
- **Tutorials** - Create step-by-step guides for common workflows
- **API Documentation** - Document the core API and platform integrations
- **Translation** - Help translate documentation into other languages

### Community Contributions
- **Bug Reports** - Report issues you encounter while using World Foundry
- **Feature Requests** - Suggest new features and improvements
- **Community Support** - Help other users in discussions and forums
- **Testing** - Test new features and provide feedback
- **Content Creation** - Share your created worlds and techniques

## 🚀 Getting Started

### 1. Set Up Your Development Environment

Follow the appropriate setup guide for your platform:
- [General Development Setup](../development/setup/DEVELOPMENT_SETUP.md)
- [Windows Development](../development/setup/WINDOWS_DEVELOPMENT.md)
- [iOS Development](../development/setup/IOS_DEVELOPMENT.md)
- [Android Development](../development/setup/ANDROID_DEVELOPMENT.md)

### 2. Understand the Project Structure

```
WorldFoundry/
├── core/                    # Rust core engine
├── platforms/              # Platform-specific implementations
│   ├── windows/            # WinUI 3 application
│   ├── ios/                # SwiftUI application
│   └── android/            # Jetpack Compose application
├── docs/                   # Documentation
├── tests/                  # Test suites
├── examples/               # Example code and worlds
└── tools/                  # Development tools and scripts
```

### 3. Read the Documentation

Before contributing, please read:
- [Documentation Guide](../DOCUMENTATION_GUIDE.md) - Standards for documentation
- [Architecture Overview](../architecture/ARCHITECTURE.md) - System design
- [Data Model](../architecture/DATA_MODEL.md) - Data structures and relationships

## 📋 Contribution Process

### 1. Find or Create an Issue

- Browse [existing issues](https://github.com/Balexda/WorldFoundry/issues) to find something to work on
- Look for issues labeled `good first issue` if you're new to the project
- Create a new issue if you've found a bug or have a feature request
- Comment on the issue to let others know you're working on it

### 2. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/WorldFoundry.git
cd WorldFoundry

# Add the upstream repository
git remote add upstream https://github.com/Balexda/WorldFoundry.git
```

### 3. Create a Branch

```bash
# Create a new branch for your contribution
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 4. Make Your Changes

- Follow the coding standards for each platform (see below)
- Write tests for new functionality
- Update documentation as needed
- Ensure your changes don't break existing functionality

### 5. Test Your Changes

```bash
# Run tests for the core engine
cd core
cargo test

# Run platform-specific tests
# Windows
cd platforms/windows
# Run tests in Visual Studio or via command line

# iOS
cd platforms/ios
xcodebuild test -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro'

# Android
cd platforms/android
./gradlew test
```

### 6. Commit Your Changes

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add settlement population growth simulation

- Implement population growth algorithms based on economic factors
- Add migration patterns between settlements
- Include demographic tracking and visualization
- Add comprehensive tests for population dynamics

Fixes #123"
```

### 7. Push and Create Pull Request

```bash
# Push your branch to your fork
git push origin feature/your-feature-name

# Create a pull request on GitHub
# Fill out the pull request template with details about your changes
```

## 📝 Coding Standards

### Rust (Core Engine)

```rust
// Use standard Rust formatting
cargo fmt

// Follow Rust naming conventions
struct WorldEngine {
    current_world: Arc<RwLock<Option<World>>>,
}

impl WorldEngine {
    pub fn new() -> Self {
        Self {
            current_world: Arc::new(RwLock::new(None)),
        }
    }
    
    pub async fn create_world(&self, config: WorldConfig) -> Result<World, EngineError> {
        // Implementation
    }
}

// Use comprehensive error handling
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("World not found: {id}")]
    WorldNotFound { id: WorldId },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Document public APIs
/// Creates a new world with the specified configuration.
/// 
/// # Arguments
/// 
/// * `config` - World generation configuration
/// 
/// # Returns
/// 
/// Returns a `Result` containing the new `World` or an `EngineError`.
/// 
/// # Examples
/// 
/// ```rust
/// let config = WorldConfig::default();
/// let world = engine.create_world(config).await?;
/// ```
pub async fn create_world(&self, config: WorldConfig) -> Result<World, EngineError> {
    // Implementation
}
```

### C# (Windows/WinUI 3)

```csharp
// Follow C# naming conventions
public class WorldEngineService
{
    private readonly ICoreEngine _coreEngine;
    
    public WorldEngineService(ICoreEngine coreEngine)
    {
        _coreEngine = coreEngine ?? throw new ArgumentNullException(nameof(coreEngine));
    }
    
    public async Task<World> CreateWorldAsync(WorldConfig config)
    {
        if (config == null)
            throw new ArgumentNullException(nameof(config));
            
        try
        {
            return await _coreEngine.CreateWorldAsync(config);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create world with config {Config}", config);
            throw;
        }
    }
}

// Use XML documentation
/// <summary>
/// Creates a new world with the specified configuration.
/// </summary>
/// <param name="config">The world generation configuration.</param>
/// <returns>A task that represents the asynchronous operation. The task result contains the created world.</returns>
/// <exception cref="ArgumentNullException">Thrown when config is null.</exception>
/// <exception cref="WorldCreationException">Thrown when world creation fails.</exception>
public async Task<World> CreateWorldAsync(WorldConfig config)
{
    // Implementation
}
```

### Swift (iOS/SwiftUI)

```swift
// Follow Swift naming conventions
class WorldEngineService: ObservableObject {
    @Published var currentWorld: World?
    @Published var isLoading = false
    
    private let coreEngine: CoreEngine
    
    init(coreEngine: CoreEngine) {
        self.coreEngine = coreEngine
    }
    
    @MainActor
    func createWorld(config: WorldConfig) async throws -> World {
        isLoading = true
        defer { isLoading = false }
        
        do {
            let world = try await coreEngine.createWorld(config: config)
            currentWorld = world
            return world
        } catch {
            logger.error("Failed to create world: \(error)")
            throw WorldCreationError.creationFailed(error)
        }
    }
}

// Use documentation comments
/// Creates a new world with the specified configuration.
/// 
/// - Parameter config: The world generation configuration
/// - Returns: The created world
/// - Throws: `WorldCreationError` if world creation fails
@MainActor
func createWorld(config: WorldConfig) async throws -> World {
    // Implementation
}
```

### Kotlin (Android/Jetpack Compose)

```kotlin
// Follow Kotlin naming conventions
class WorldEngineService @Inject constructor(
    private val coreEngine: CoreEngine
) {
    private val _currentWorld = MutableStateFlow<World?>(null)
    val currentWorld: StateFlow<World?> = _currentWorld.asStateFlow()
    
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()
    
    suspend fun createWorld(config: WorldConfig): Result<World> = withContext(Dispatchers.IO) {
        _isLoading.value = true
        
        try {
            val world = coreEngine.createWorld(config)
            _currentWorld.value = world
            Result.success(world)
        } catch (e: Exception) {
            logger.error("Failed to create world", e)
            Result.failure(WorldCreationException("World creation failed", e))
        } finally {
            _isLoading.value = false
        }
    }
}

/**
 * Creates a new world with the specified configuration.
 * 
 * @param config The world generation configuration
 * @return Result containing the created world or an error
 */
suspend fun createWorld(config: WorldConfig): Result<World> {
    // Implementation
}
```

## 🧪 Testing Guidelines

### Unit Tests

Write comprehensive unit tests for all new functionality:

```rust
// Rust tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_world_creation() {
        let engine = WorldEngine::new();
        let config = WorldConfig::default();
        
        let result = engine.create_world(config).await;
        
        assert!(result.is_ok());
        let world = result.unwrap();
        assert!(!world.name.is_empty());
        assert!(world.map.width > 0);
        assert!(world.map.height > 0);
    }
    
    #[test]
    fn test_invalid_config() {
        let engine = WorldEngine::new();
        let config = WorldConfig {
            width: 0,  // Invalid
            height: 0, // Invalid
            ..Default::default()
        };
        
        let result = tokio_test::block_on(engine.create_world(config));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EngineError::InvalidConfig(_)));
    }
}
```

### Integration Tests

Test interactions between components:

```rust
#[tokio::test]
async fn test_azgaar_import_integration() {
    let engine = WorldEngine::new();
    let test_file = "tests/data/sample_azgaar.json";
    
    let result = engine.import_azgaar(test_file).await;
    
    assert!(result.is_ok());
    let world = result.unwrap();
    assert!(!world.cultures.is_empty());
    assert!(!world.settlements.is_empty());
}
```

### Platform-Specific Tests

Each platform should have comprehensive UI and integration tests.

## 📚 Documentation Standards

Follow the [Documentation Guide](../DOCUMENTATION_GUIDE.md) for all documentation contributions:

### Code Documentation

- Document all public APIs
- Include examples in documentation
- Explain complex algorithms and design decisions
- Keep documentation up to date with code changes

### User Documentation

- Write clear, step-by-step instructions
- Include screenshots and diagrams where helpful
- Test all instructions to ensure they work
- Consider different skill levels of users

## 🔍 Code Review Process

### What We Look For

- **Correctness**: Does the code work as intended?
- **Performance**: Is the code efficient and scalable?
- **Security**: Are there any security vulnerabilities?
- **Maintainability**: Is the code easy to understand and modify?
- **Testing**: Are there adequate tests for the changes?
- **Documentation**: Is the code and functionality properly documented?

### Review Guidelines

- Be respectful and constructive in feedback
- Explain the reasoning behind suggestions
- Ask questions if something is unclear
- Acknowledge good work and improvements
- Focus on the code, not the person

## 🐛 Bug Reports

When reporting bugs, please include:

### Bug Report Template

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
 - OS: [e.g. Windows 11, macOS 14, Android 14]
 - Version: [e.g. 1.0.0]
 - Device: [e.g. iPhone 15, Pixel 8, Surface Pro]

**Additional context**
Add any other context about the problem here.

**Log files**
If available, attach relevant log files or error messages.
```

## 💡 Feature Requests

When requesting features, please include:

### Feature Request Template

```markdown
**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.

**Use cases**
Describe specific use cases where this feature would be helpful.

**Implementation ideas**
If you have ideas about how this could be implemented, please share them.
```

## 🏆 Recognition

We value all contributions to World Foundry! Contributors will be:

- Listed in the project's contributors file
- Mentioned in release notes for significant contributions
- Invited to join the core contributor team for ongoing contributions
- Featured in community showcases for exceptional work

## 📞 Getting Help

If you need help with contributing:

- **GitHub Discussions**: Ask questions and discuss ideas
- **Discord Server**: Real-time chat with the community
- **Documentation**: Check the comprehensive documentation
- **Issues**: Search existing issues or create a new one

## 📜 Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to providing a welcoming and inclusive environment for all contributors.

## 📄 License

By contributing to World Foundry, you agree that your contributions will be licensed under the same license as the project. See the [LICENSE](../../LICENSE) file for details.

---

Thank you for contributing to World Foundry! Your efforts help make fantasy worldbuilding accessible and enjoyable for everyone. 🌍✨

**Last Updated**: March 2024  
**Contributing Guide Version**: 1.0  
**Project Repository**: [github.com/Balexda/WorldFoundry](https://github.com/Balexda/WorldFoundry)