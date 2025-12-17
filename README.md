# randwri

A fast utility to write random data to files or system devices using a non-cryptographic random number generator.

## Features

- **High performance**: Uses `SmallRng` for fast random data generation
- **System device support**: Can write to block devices and special files (e.g., `/dev/sdX`)
- **Human-friendly sizes**: Supports K, M, G, T suffixes for size specification
- **Progress reporting**: Real-time progress bar with speed and ETA
- **Configurable buffer**: Adjustable buffer size for optimal performance

## Installation

### From Source

```bash
git clone <repository-url>
cd randwri
cargo build --release
```

The binary will be available at `target/release/randwri`.

## Usage

```bash
randwri [OPTIONS] --output <OUTPUT>
```

### Options

- `-o, --output <OUTPUT>` - Output file or device path (required)
- `-s, --size <SIZE>` - Size of data to write (default: `1G`)
  - Supports suffixes: `K` (1024), `M` (1024²), `G` (1024³), `T` (1024⁴)
  - Examples: `500K`, `100M`, `2G`, `1T`
- `-b, --buffer-size <BUFFER_SIZE>` - Buffer size for writing (default: `1M`)
  - Supports K, M suffixes
  - Larger buffers may improve performance on fast storage

### Examples

```bash
# Write 1GB to a file (default size)
randwri -o test.bin

# Write 500MB to a file
randwri -o test.bin -s 500M

# Write 2GB with a 4MB buffer
randwri -o test.bin -s 2G -b 4M

# Write to system device
randwri -o /dev/null -s 1G

# Write to block device
randwri -o /dev/sdb -s 10G
```

## Performance Tips

- Use larger buffer sizes (`-b 4M` or `-b 8M`) for faster storage devices
- The default 1MB buffer works well for most use cases
- Progress updates occur every 1MB to minimize overhead

## Notes

- Uses non-cryptographic random number generator (`SmallRng`) - suitable for performance testing, not for security-sensitive applications
- Does not truncate files/devices - overwrites from the beginning
- Supports both regular files and system devices (block devices, `/dev/null`, etc.)

