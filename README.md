<h1 align=center>
    rt
    <br>
    <img alt="Ferris" src="assets/img/ferris.svg">
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Tech Stack](#tech-stack)
- [Overview](#overview)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File System](#file-system)
- [Architecture](#architecture)
  - [Entities](#entities)
  - [Sequence](#sequence)
- [Usage](#usage)
- [Create objects](#create-objects)
  - [Sphere](#sphere)
  - [Cube](#cube)
  - [Cylinder](#cylinder)
  - [Flat plane](#flat-plane)
- [Mechanism](#mechanism)
  - [Camera](#camera)
  - [Ray](#ray)
- [Contributors](#contributors)
  - [Authors](#authors)
  - [Peers](#peers)
  - [Testers](#testers)
  - [Auditors](#auditors)
- [Sources](#sources)
- [License](#license)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./scripts/gitify.sh)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)
![WARP](https://img.shields.io/badge/warp-01A4FF?style=for-the-badge&logo=warp&logoColor=white)
![LINUX](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![MAC OS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)

## Overview

As a `Ray Tracer`, this program renders `3D` scenes into a `2D` images by drawing each pixel with its color, shadows, reflection, refraction and other parameters using.
In the context of `3D` computer graphics, ray tracing is a method that simulates light transport in a wide variety of rendering algorithms to generate digital images in `2D`.

<div align=center>
    <img alt="rt" src="assets/img/scene_3.png">
</div>

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
      +-📂 assets/
      |       |
      |       +-⚙️ rustfmt.toml
      |
      +-📂 assets/
      |       |
      |       +---📂 img/
      |       |       |
      |       |       +-🏞 ferris.svg
      |       |       +-🏞 ray_trace_diagram.svg
      |       |       +-🏞 rays_viewport_schema.png
      |       |       +-🏞 scene_3.png
      |       |
      |       +-📂 scenes/
      |               |
      |               +-🏞 001.ppm
      |               +-🏞 002.ppm
      |               +-🏞 003.ppm
      |               +-🏞 004.ppm
      |
      +-📂 scripts/
      |       |
      |       +-📜 gitify.sh
      |       +-📜 utils.sh
      |
      +---📂 src/
      |       |
      |       +-📂 camera/
      |       |       |
      |       |       +-📜 builder.rs
      |       |       +-📜 mod.rs
      |       |       +-📜 model.rs
      |       |       +-📜 utils.rs
      |       |
      |       +-📂 utils/
      |       |       |
      |       |       +-📜 constants.rs
      |       |       +-📜 error.rs
      |       |       +-📜 mod.rs
      |       |       +-📜 utils.rs
      |       |
      |       +-📂 geometry/
      |       |       |
      |       |       +-📂 objects/
      |       |       |       |
      |       |       |       +-📜 cube.rs
      |       |       |       +-📜 cylinder.rs
      |       |       |       +-📜 mod.rs
      |       |       |       +-📜 plane.rs
      |       |       |       +-📜 sphere.rs
      |       |       |
      |       |       +-📂 vectors/
      |       |       |       |
      |       |       |       +-📜 mod.rs
      |       |       |       +-📜 mutation.rs
      |       |       |       +-📜 scalar_ops.rs
      |       |       |       +-📜 vector_ops.rs
      |       |       |
      |       |       +-📜 mod.rs
      |       |
      |       +-📂 graphics/
      |       |       |
      |       |       +-📂 scene/
      |       |       |       |
      |       |       |       +-📜 builder.rs
      |       |       |       +-📜 mod.rs
      |       |       |       +-📜 model.rs
      |       |       |
      |       |       +-📜 image.rs
      |       |       +-📜 mod.rs
      |       |
      |       +-📂 material/
      |       |       |
      |       |       +-📜 dielectric.rs
      |       |       +-📜 emissive.rs
      |       |       +-📜 lambertian.rs
      |       |       +-📜 metal.rs
      |       |       +-📜 mod.rs
      |       |
      |       +-📂 optics/
      |       |       |
      |       |       +-📜 impact.rs
      |       |       +-📜 mod.rs
      |       |       +-📜 ray.rs
      |       |
      |       +-📜 lib.rs
      |       +-📜 main.rs
      |
      |
      +-📂 tests/
      |       |
      |       +-📜 vector_test.rs
      |
      +-📂 todos/
      |       |
      |       +-📜 audit.todo
      |       +-📜 tasks.todo
      |
      +-🚫 .gitignore
      +-🔒 Cargo.lock
      +-⚙️ Cargo.toml
      +-🔑 LICENSE
      +-📖 README.md

## Architecture

```mermaid
architecture-beta
  group rt(logos:rust)[Ray Tracer]

    group src(logos:rust)[Source] in rt
      service cam(logos:google-meet)[Camera] in src

      group geometry(logos:rust)[Geometry] in src
        service obj(logos:apostrophe)[Objects] in geometry
        service vec(logos:appbase)[Vector] in geometry

      group graphics(logos:rust)[Graphics] in src
        service img(logos:imagemin)[Image] in graphics
        service scene(logos:google-play-console-icon)[Scene] in graphics

      group optics(logos:rust)[Optics] in src
        service ray(logos:spark)[Ray] in optics
        service impact(logos:launchdarkly-icon)[Impact] in optics

    group assets(disk)[Assets] in rt
      service output(logos:google-keep)[Output] in assets

    service input(logos:bash-icon)[Input] in rt

  input:R --> L:scene{group}
  img:T --> B:scene
  cam:L --> R:scene{group}
  obj{group}:B --> T:scene{group}
  img{group}:B --> T:output{group}

  vec:L --> R:obj
  vec{group}:B --> T:cam

  ray:L --> R:cam
  impact:B --> T:ray
```

```mermaid
graph TD
    A[Input] --> B{Scene}
    B --> C[Ray Generation]
    C --> D[Intersection]
    D --> E[Shading]
    E --> F[Image]
```

### Entities

```mermaid
classDiagram
%% direction LR

%% namespace Graphics {
  class Image {
    <<struct>>
    -width
    -height
    -px_colors
    +new(width, height) Image
    +set_px_color(row, col, color)
    +write_ppm(output_file)
  }

  class Scene {
    <<struct>>
    -id
    -camera
    -lights
    -objects
    +new(id, camera, lights, objects) Scene
    +display()
  }
%% }

%% namespace Optics {
  class Camera {
    <<struct>>
    -origin
    -bottom_left
    -horizontal
    -vertical
    -look_from
    -look_at
    +new(origin) Camera
    +get_ray(u, v) Ray
  }

  class Light {
    <<struct>>
    -position
    -color
    -intensity
    +intensity() f64
    +new(position, color, intensity) Light
    +illuminate(impact, objects) Color
  }

  class Ray {
    <<struct>>
    -origin
    -direction
    +new(origin, direction) Ray
    +cast(t) Position
    +color(objects, lights, depth) Color
  }

  class Impact {
    <<struct>>
    +point
    +surface_normal
    +t
    +new() Impact
    +set_face_normal(incident, outward)
  }
%% }

%% namespace Objects {
  class Object {
    <<trait>>
    +color() Color
    +position() Position
    +hit(ray, t_min, t_max, impact) bool
    +depth() i32
  }

  class Sphere {
    <<struct>>
    -center
    -radius
    -color
    +new() Sphere
  }

  class Cube {
    <<struct>>
    -side
    -position
    -color
    +new() Cube
  }

  class Cylinder {
    <<struct>>
    -center
    -radius
    -height
    -orientation
    -color
    +new() Cylinder
  }

  class Plane {
    <<struct>>
    -position
    -normal
    -color
    +new() Plane
  }
%% }

%% namespace Vectors {
  class Vector {
    <<struct>>
    +new(x, y, z) Vector
    +random() Vector
    +random_range(min, max) Vector
    +random_unit() Vector
    +x() f64
    +y() f64
    +z() f64
    +dot(other) f64
    +length() f64
    +unit() Vector
  }

  class Color {
    <<type>>
  }

  class Position {
    <<type>>
  }

  class Direction {
    <<type>>
  }
%% }

Scene --> Camera: Has
Scene --> Light: Has
Scene --> Object: Has

Image --* Scene: Generated by

Camera o-- Vector: Position
Camera o-- Vector: Direction
Camera --> Ray: Generates

Ray o-- Vector: Position
Ray o-- Vector: Direction
Impact --* Ray: Generated by
Ray --> Object: Casts

Copy ()-- Impact
Clone ()-- Impact
Default ()-- Impact
Impact o-- Vector: Position
Impact o-- Vector: Direction
Impact <-- Object: Updated by

Object o-- Vector: Position
Object o-- Vector: Color
Object <|.. Sphere: Implements
Object <|.. Cube: Implements
Object <|.. Cylinder: Implements
Object <|.. Plane: Implements

Cylinder o-- Vector: Direction

Plane o-- Vector: Direction

Light o-- Vector: Color
Light o-- Vector: Position
Light --> Ray: Generates
Light --> Impact: Diffused by

Position ..> Vector: Is
Direction ..> Vector: Is
Color ..> Vector: Is

Vector --() Display
Vector --() Neg
Vector --() AddAssign
Vector --() MulAssign_f64
Vector --() DivAssign_f64
Vector --() Add
Vector --() Sub
Vector --() Mul
Vector --() Mul_f64
Vector --() Div_f64
Vector --() Debug
Vector --() Clone
Vector --() Copy
Vector --() Default
Vector --() PartialEq
Vector --() PartialOrd

style Scene fill:#bfb,color:#000
style Image fill:#bfb,color:#000

style Camera fill:#ffb,color:#000
style Light fill:#ffb,color:#000
style Ray fill:#ffb,color:#000
style Impact fill:#ffb,color:#000

style Object fill:#9ff,color:#000
style Sphere fill:#9ff,color:#000
style Cube fill:#9ff,color:#000
style Cylinder fill:#9ff,color:#000
style Plane fill:#9ff,color:#000

style Vector fill:#fbb,color:#000
style Position fill:#fbb,color:#000
style Direction fill:#fbb,color:#000
style Color fill:#fbb,color:#000
```

### Sequence

```mermaid
sequenceDiagram
title Scene::display()
%% autonumber

  participant Scene
  participant Camera
  participant Light
  participant Objects

  activate Scene
  create participant Image
  Scene ->>+ Image: ::new()
  Image -->>- Scene: Image
  loop Pixel Rows
    loop Pixel Columns
      create participant Color
      Scene ->>+ Color: ::default()
      Color -->>- Scene: Color

      loop Ray Samples per Pixel
        Scene ->>+ Camera: .get_ray()

        create participant Ray
        Camera ->>+ Ray: ::new()
        Ray -->>- Camera: Ray
        Camera -->>- Scene: Ray
        Scene ->>+ Ray: .color()

        create participant Impact
        Ray ->>+ Impact: ::new()
        Impact -->>- Ray: Impact
        Ray ->>+ Objects: Option<>

        loop Each Object
          Ray ->>+ Impact: ::new()
          Impact -->>- Ray: Impact
          Ray ->>+ Objects: .hit()
          Objects -->>- Ray: bool

          opt got_hit
          rect rgb(50,50,0,.5)
            Ray ->> Impact: Updates
            Ray ->> Objects: Updates
          end
          end

          alt Some(object)
          rect rgb(0,25,0,.25)
            Ray ->>+ Color: ::default()
            Color -->>- Ray: Color
            Ray ->>+ Color: ::new()
            Color -->>- Ray: Color

            loop Each Light
            rect rgb(0,0,25,.25)
              Ray ->>+ Light: .illuminate()
              Light ->>+ Ray: ::new()
              Ray -->>- Light: Ray

              loop Each Object
              rect rgb(0,0,25,.25)
                Light ->>+ Objects: .hit()
                Objects -->>- Light: bool

                alt got_hit
                rect rgb(0,25,0,.5)
                  Light ->>+ Color: ::default()
                  Color -->>- Light: Color
                end
                else
                rect rgb(25,0,0.25)
                  destroy Impact
                  Light ->> Impact: Diffuse
                end
                end
              end
              end
            end
            end
            Light -->>- Ray: Color
          end
          else
          rect rgb(25,0,0,.25)
            Ray ->>+ Color: ::new()
            Color -->>- Ray: Color
          end
          end
        end

        deactivate Ray
        destroy Ray
        Ray -->> Scene: Color
      end

      Scene ->>+ Image: .set_px_color()
      Image ->>- Color: Updates
    end
  end

  Scene ->>+ Image: .write_ppm()
  destroy Color
  Image ->>- Color: writeln!()
  deactivate Scene
```

## Usage

```shell
cargo run > assets/output.ppm
```

## Create objects

### [Sphere](./src/geometry/objects/sphere.rs)

First of all, the condition for a given point to be considered as being on the sphere is to have its **coordonates' absolute value** `equal` to the sphere's **radius**, better illustrated with the equation: $x^2 + y^2 + z^2 = R^2$. Given the center of the sphere the equation, that specific point's coordonates will be the difference between its **coordonates** and the sphere's **center** `C`:

$$
\\[25pt] \huge (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2 = r^2 \\[50pt]
$$

Considering these coordonates as part of a vector, those `x,y,z` operations can be shortcut to a **difference** between the given position `P` and de sphere's center `C`:

$$
\large (\vec{P}_{(x, y, z)} - \vec{C}_{(x, y, z)})\cdot(\vec{P}_{(x, y, z)} - \vec{C}_{(x, y, z)}) = (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2 = r^2 \\[15pt]
\Downarrow \\[15pt]
\huge (\vec{P} - \vec{C})\cdot(\vec{P} - \vec{C}) = r^2 \\[50pt]
$$

Now from the `ray casting` function, $P(t) = A + tb$, the point resulting from `t` should satify the contidion to be considered as hitting the sphere:

$$
\large (\vec{P}(t) - \vec{C}) \cdot (\vec{P}(t) - \vec{C}) = r^2 \\[15pt]
\Downarrow \\[15pt]
\large (\vec{A} + t\vec{b} - \vec{C}) \cdot (\vec{A} + t\vec{b} - \vec{C}) = r^2 \\[15pt]
\Downarrow \\[15pt]
\Large t^2\vec{b}^2 + 2t\vec{b} \cdot (\vec{A} - \vec{C}) + (\vec{A} - \vec{C}) \cdot (\vec{A} - \vec{C}) - r^2 = 0 \\[50pt]
$$

Finally, since $\large t$ is the only unknown, the `variable` so to say, and given that the equation is `quadratic`, $\large t$ can be solve using the quadratic formula:

$$
\\[25pt] \huge t = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} \\[-20pt]
$$

$\small Where:\\$
$\small a = \vec{b}^2\\$
$\small b = 2\vec{b} \cdot (\vec{A} - \vec{C})\\$
$\small c = (\vec{A} - \vec{C}) \cdot (\vec{A} - \vec{C}) - r^2\\[15pt]$

$$
b = 2h: \\[15pt]
\Downarrow \\[15pt]
\large t = \frac{-2h \pm \sqrt{(2h)^2 - 4ac}}{2a} \\[15pt]
\Downarrow \\[15pt]
\large t = \frac{-2h \pm 2\sqrt{h^2 - ac}}{2a} \\[15pt]
\Downarrow \\[25pt]
\Huge t = \frac{-h \pm \sqrt{h^2 - ac}}{a} \\[25pt]
$$

The `discriminant` ($h^2 - ac$), helps to identify how many intersection points exist between the ray and the sphere:

- `discriminant > 0`: There are two distinct intersection points. This means the ray enters and exits the sphere.

- `discriminant == 0`: There is exactly one intersection point (the ray is tangent to the sphere). This means the ray just touches the sphere at one point.

- `discriminant < 0`: There are no intersection points. This means the ray does not intersect the sphere at all.

### [Cube](./src/geometry/objects/cube.rs)

### [Cylinder](./src/geometry/objects/cylinder.rs)

### [Flat plane](./src/geometry/objects/plane.rs)

## Mechanism

### [Camera](./src/camera/model.rs)

<figure align=center>
    <img alt="rays_schemas" src="assets/img/rays_viewport_schema.png">
    <br>
    <figcaption>By <a href="//utilss.wikimedia.org/w/index.php?title=User:Kamil_Kielczewski&amp;action=edit&amp;redlink=1" class="new" title="User:Kamil Kielczewski (page does not exist)">Kamil Kielczewski</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativeutilss.org/licenses/by-sa/4.0" title="Creative utilss Attribution-Share Alike 4.0">CC BY-SA 4.0</a>, <a href="https://utilss.wikimedia.org/w/index.php?curid=76049175">Link</a></figcaption>
</figure>

### [Ray](./src/optics/ray.rs)

<figure align=center>
    <img alt="rt_diagram" src="assets/img/ray_trace_diagram.svg">
    <br>
    <figcaption>By <a href="//utilss.wikimedia.org/wiki/User:Henrik" title="User:Henrik">Henrik</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativeutilss.org/licenses/by-sa/4.0" title="Creative utilss Attribution-Share Alike 4.0">CC BY-SA 4.0</a>, <a href="https://utilss.wikimedia.org/w/index.php?curid=3869326">Link</a></figcaption>
</figure>

## Contributors

### Authors

[![ndiediop](https://shields.io/badge/Author-ndiediop-magenta)](http://learn.zone01dakar.sn/git/ndiediop)
[![npouille](https://shields.io/badge/Author-npouille-magenta)](http://learn.zone01dakar.sn/git/npouille)
[![papebsow](https://shields.io/badge/Author-papebsow-cyan)](http://learn.zone01dakar.sn/git/papebsow)
[![jefaye](https://shields.io/badge/Author-jefaye-cyan)](http://learn.zone01dakar.sn/git/jefaye)

### Peers

[![jgoudiab](https://shields.io/badge/Zone01-jgoudiab-blue)](http://learn.zone01dakar.sn/git/jgoudiab)

### Testers

### Auditors

## Sources

[![YOUTUBE](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)]()

[![WIKI](https://shields.io/badge/Ray_tracing-Wikipedia-white)](<https://en.wikipedia.org/wiki/Ray_tracing_(graphics)>)

## License

[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)
