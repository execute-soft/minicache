const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For non-Linux platforms, return false
  if (!process.platform === 'linux') {
    return false
  }
  // For Linux, detect if we're using musl
  return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
}

function loadBinding() {
  let platformName

  /* istanbul ignore next */
  switch (platform) {
    case 'android':
      switch (arch) {
        case 'arm64':
          platformName = 'android-arm64'
          break
        case 'arm':
          platformName = 'android-arm-eabi'
          break
        default:
          throw new Error(`Unsupported architecture on Android ${arch}`)
      }
      break
    case 'win32':
      switch (arch) {
        case 'x64':
          platformName = 'win32-x64-msvc'
          break
        case 'ia32':
          platformName = 'win32-ia32-msvc'
          break
        case 'arm64':
          platformName = 'win32-arm64-msvc'
          break
        default:
          throw new Error(`Unsupported architecture on Windows: ${arch}`)
      }
      break
    case 'darwin':
      switch (arch) {
        case 'x64':
          platformName = 'darwin-x64'
          break
        case 'arm64':
          platformName = 'darwin-arm64'
          break
        default:
          throw new Error(`Unsupported architecture on macOS: ${arch}`)
      }
      break
    case 'freebsd':
      if (arch !== 'x64') {
        throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
      }
      platformName = 'freebsd-x64'
      break
    case 'linux':
      switch (arch) {
        case 'x64':
          if (isMusl()) {
            platformName = 'linux-x64-musl'
          } else {
            platformName = 'linux-x64-gnu'
          }
          break
        case 'arm64':
          if (isMusl()) {
            platformName = 'linux-arm64-musl'
          } else {
            platformName = 'linux-arm64-gnu'
          }
          break
        case 'arm':
          platformName = 'linux-arm-gnueabihf'
          break
        default:
          throw new Error(`Unsupported architecture on Linux: ${arch}`)
      }
      break
    default:
      throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
  }

  try {
    // Try to load the locally built binary first
    const localPath = join(__dirname, `index.${platformName}.node`)
    if (existsSync(localPath)) {
      localFileExisted = true
      nativeBinding = require(localPath)
    } else {
      // Try to load from npm package
      nativeBinding = require(`minicache-${platformName}`)
    }
  } catch (e) {
    loadError = e
    throw e
  }
}

loadBinding()

module.exports = nativeBinding
