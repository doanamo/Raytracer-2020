Raytracer ![](https://github.com/gunstarpl/Raytracer-2019/workflows/Build/badge.svg)
====
Raytracer written in Rust as a learning experience. Implementation is based on material from following books:
* [Ray Tracing in One Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html)
* [Physically Based Rendering: From Theory To Implementation](http://www.pbr-book.org/)

Usage
====
1. Install Rust 1.40.0 from https://www.rust-lang.org/
2. Clone repository and run ```cargo run --release -- "examples/spheres.json"```
3. Output image will be saved as ```renders/spheres.png``` file

Showcase
====
![](renders/spheres.png)
![](renders/metallic.png)
![](renders/focus.png)

License
====
The MIT License (MIT)

Copyright (c) 2019-2020 Piotr Doan

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
