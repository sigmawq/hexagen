## hexagen
Simple tool for creating hexagon templates in PNG format. Hex itself is white and everything else is alpha. 
Having a generic hex template makes it easy to build textured hex grids.

# Installation
1. Make sure rust compiler is installed
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone repository
```
git clone https://github.com/sigmawq/hexagen.git
```

3. Compile project
```
cargo build --release
```

Compiled binary will be in ` target/release/ `

# Usage 
```
hexagen <hex-radius> <output-path> <hex-type>
```
` hex-type ` is either 'Flat' or 'Pointy'

# Example
Generate a template flat hexagon with radius 72.0
```
hexagen 72.0 hex.png Flat
```
Result:
 
![Image alt text](hex.png?raw=true)
