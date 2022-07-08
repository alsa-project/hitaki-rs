================================
Rust bindings for hitaki library
================================

2022/07/08
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhitaki`` which provides ``Hitaki-0.0.gir``.

  * https://github.com/alsa-project/libhitaki

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* The latest release is version 0.0.91. This is pre-release to publish crates in crates.io.

License
=======

MIT License

Dependencies
============

* `libhitaki <https://github.com/takaswie/libhitaki>`_
* FFI crate (``hitaki-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crate (``hitaki``)

  * ``libc`` >= 0.2
  * ``bitflags`` >= 1.0
  * ``glib`` >= 0.15
  * FFI crate (``hitaki-sys``)
