<h1 align=center>
    <img alt="Ferris" src="assets/ferris.svg">
    <br>
    rt
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Tech Stack](#tech-stack)
- [Overview](#overview)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File System](#file-system)
- [Usage](#usage)
  - [Mechanism](#mechanism)
  - [Calculate rays for rectangular viewport](#calculate-rays-for-rectangular-viewport)
  - [Create objects](#create-objects)
    - [Sphere](#sphere)
    - [Cube](#cube)
    - [Cylinder](#cylinder)
    - [Flat plane](#flat-plane)
- [Contributors](#contributors)
  - [Collaborators](#collaborators)
  - [Peers](#peers)
  - [Testers](#testers)
  - [Auditors](#auditors)
- [License](#license)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./scripts/gitify.sh)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)
![WARP](https://img.shields.io/badge/warp-01A4FF?style=for-the-badge&logo=warp&logoColor=white)
![LINUX](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![MAC OS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)

## Overview

<div align=center><img alt="rt" src="assets/raytrace.png"></div>

## Installation

### Cloning

```shell
git clone http://learn.zone01dakar.sn/git/jefaye/rt
Cloning into 'rt'...
Username for 'https://learn.zone01dakar.sn': jefaye
Password for 'https://jefaye@learn.zone01dakar.sn':
warning: redirecting to https://learn.zone01dakar.sn/git/jefaye/rt/
remote: Enumerating objects: 86, done.
remote: Counting objects: 100% (86/86), done.
remote: Compressing objects: 100% (65/65), done.
remote: Total 86 (delta 12), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (86/86), 273.12 KiB | 8.28 MiB/s, done.
Resolving deltas: 100% (12/12), done.

cd rt
tree --dirsfirst
```

### File System

    📂./
      |
      +- 📂 assets/
      |       |
      |       +- 🌄 ferris.svg
      |       +- 🌄 ray_trace_diagram.svg
      |       +- 🌄 rays_viewport_schema.png
      |       +- 🌄 raytrace.png
      |
      +- 📂 scripts/
      |       |
      |       +- 📜 gitify.sh
      |       +- 📜 utils.sh
      |
      +-- 📂 src/
      |       |
      |       +- 📂 camera/
      |       |       |
      |       |       +- 📄 camera.rs
      |       |       +- 📄 mod.rs
      |       |
      |       +- 📂 geometry/
      |       |       |
      |       |       +- 📂 objects/
      |       |       |       |
      |       |       |       +- 📄 cube.rs
      |       |       |       +- 📄 cylinder.rs
      |       |       |       +- 📄 mod.rs
      |       |       |       +- 📄 plane.rs
      |       |       |       +- 📄 sphere.rs
      |       |       |
      |       |       +- 📂 vectors/
      |       |               |
      |       |               +- 📄 mod.rs
      |       |               +- 📄 traits.rs
      |       |
      |       +- 📂 light/
      |       |       |
      |       |       +- 📄 light.rs
      |       |       +- 📄 mod.rs
      |       |
      |       +- 📂 renderer/
      |       |       |
      |       |       +- 📄 mod.rs
      |       |       +- 📄 ray_tracer.rs
      |       |       +- 📄 scene.rs
      |       |
      |       +- 📄 lib.rs
      |       +- 📄 main.rs
      |
      |
      +- 📂 tests/
      |       |
      |       +- 📄 vector_test.rs
      |
      +- 📂 todos/
      |       |
      |       +- 📝 audit.todo
      |       +- 📝 tasks.todo
      |
      +- 🚫 .gitignore
      +- 🔒 Cargo.lock
      +- ⚙️ Cargo.toml
      +- 🔑 LICENSE
      +- 📖 README.md
      +- ⚙️ rustfmt.toml

## Usage

```shell
cargo run > scenes/output.ppm
```

### Mechanism

<figure align=center>
    <img alt="rt_diagram" src="assets/ray_trace_diagram.svg">
    <br>
    <figcaption>By <a href="//commons.wikimedia.org/wiki/User:Henrik" title="User:Henrik">Henrik</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativecommons.org/licenses/by-sa/4.0" title="Creative Commons Attribution-Share Alike 4.0">CC BY-SA 4.0</a>, <a href="https://commons.wikimedia.org/w/index.php?curid=3869326">Link</a></figcaption>
</figure>

### Calculate rays for rectangular viewport

<figure align=center>
    <img alt="rays_schemas" src="assets/rays_viewport_schema.png">
    <br>
    <figcaption>By <a href="//commons.wikimedia.org/w/index.php?title=User:Kamil_Kielczewski&amp;action=edit&amp;redlink=1" class="new" title="User:Kamil Kielczewski (page does not exist)">Kamil Kielczewski</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativecommons.org/licenses/by-sa/4.0" title="Creative Commons Attribution-Share Alike 4.0">CC BY-SA 4.0</a>, <a href="https://commons.wikimedia.org/w/index.php?curid=76049175">Link</a></figcaption>
</figure>

### Create objects

#### Sphere

#### Cube

#### Cylinder

#### Flat plane

## Contributors

### Collaborators

[![ndiediop](https://shields.io/badge/Author-ndiediop-magenta)](http://learn.zone01dakar.sn/git/ndiediop)
[![npouille](https://shields.io/badge/Author-npouille-magenta)](http://learn.zone01dakar.sn/git/npouille)
[![papebsow](https://shields.io/badge/Author-papebsow-cyan)](http://learn.zone01dakar.sn/git/papebsow)
[![jefaye](https://shields.io/badge/Author-jefaye-cyan)](http://learn.zone01dakar.sn/git/jefaye)

### Peers

[![jgoudiab](https://shields.io/badge/Author-jgoudiab-blue)](http://learn.zone01dakar.sn/git/jgoudiab)

### Testers

### Auditors

## License

[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)
