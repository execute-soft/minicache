# Publishing Guide for MiniCache

This guide helps you publish the MiniCache package to crates.io.

## Prerequisites

1. **Rust and Cargo**: Ensure you have Rust 1.70+ installed
2. **Crates.io Account**: Create an account at [crates.io](https://crates.io)
3. **API Token**: Generate an API token from your crates.io account settings
4. **Git Repository**: Push your code to a public Git repository (GitHub, GitLab, etc.)

## Before Publishing

### 1. Update Repository URLs

Edit `Cargo.toml` and replace placeholder URLs:

```toml
homepage = "https://github.com/yourusername/minicache"
repository = "https://github.com/yourusername/minicache"
```

### 2. Update Author Information

```toml
authors = ["Your Name <your.email@example.com>"]
```

### 3. Final Validation

```bash
# Run all tests
cargo test

# Check for issues
cargo clippy

# Build in release mode
cargo build --release

# Test documentation
cargo doc --open

# Verify package contents
cargo package --list
```

## Publishing Steps

### 1. Login to Crates.io

```bash
cargo login <your-api-token>
```

### 2. Package and Publish

```bash
# Create the package (optional - for inspection)
cargo package

# Publish to crates.io
cargo publish
```

### 3. Verify Publication

- Visit your package page: `https://crates.io/crates/minicache`
- Check that documentation builds: `https://docs.rs/minicache`

## Post-Publication

### 1. Create a Release Tag

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 2. Create GitHub Release

- Go to your repository's releases page
- Create a new release with tag `v0.1.0`
- Copy content from `CHANGELOG.md`

### 3. Update Documentation

- Verify docs.rs built successfully
- Update README badges with correct links

## Future Releases

### Version Updates

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.1.1"
   ```

2. Update `CHANGELOG.md` with new changes

3. Commit changes:
   ```bash
   git add .
   git commit -m "Release v0.1.1"
   git tag v0.1.1
   git push origin main --tags
   ```

4. Publish new version:
   ```bash
   cargo publish
   ```

## Package Features

✅ **Complete Package Contents:**
- `src/lib.rs` - Main library with comprehensive docs
- `src/core.rs` - Core implementation with full documentation
- `README.md` - Comprehensive usage guide
- `LICENSE` - MIT license
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Complete metadata
- `examples/` - Usage examples
- `benches/` - Performance benchmarks

✅ **Quality Checks:**
- All tests pass (17 unit tests + 13 doc tests)
- Clippy warnings minimal and acceptable
- Documentation complete with examples
- Performance benchmarks included
- Memory profiling tools available

✅ **Crates.io Compatibility:**
- Proper keywords and categories
- MIT license
- Complete metadata
- Documentation links
- Repository links
- Semantic versioning

## Troubleshooting

### Publication Fails

1. **Name conflict**: Package name already exists
   - Change `name` in `Cargo.toml`
   - Try variations like `minicache-rs`, `mini-cache`, etc.

2. **Missing metadata**: Error about required fields
   - Ensure all required fields in `Cargo.toml` are filled

3. **Documentation issues**: Build failures on docs.rs
   - Test locally: `cargo doc --no-deps`
   - Fix any broken links or examples

### Version Issues

1. **Version already published**: Cannot overwrite existing versions
   - Increment version number
   - Use `cargo search minicache` to check existing versions

## Success Criteria

Your package is ready when:

- [ ] `cargo test` passes all tests
- [ ] `cargo clippy` shows only acceptable warnings
- [ ] `cargo build --release` succeeds
- [ ] `cargo package --list` shows expected files
- [ ] Documentation builds locally with `cargo doc`
- [ ] Repository URLs are correct
- [ ] Author information is updated

## Example Commands

```bash
# Complete validation sequence
cargo clean
cargo test
cargo clippy
cargo build --release
cargo doc --no-deps
cargo package --list

# Publish when ready
cargo login <token>
cargo publish
```

## Support

- [Cargo Book](https://doc.rust-lang.org/cargo/) - Complete cargo documentation
- [Crates.io Guide](https://doc.rust-lang.org/cargo/reference/publishing.html) - Publishing guide
- [API Guidelines](https://rust-lang.github.io/api-guidelines/) - Rust API best practices

---

**Ready to publish? Follow the steps above and share MiniCache with the Rust community! 🚀**