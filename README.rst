================================
Rust bindings for hitaki library
================================

2024/02/25
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhitaki`` which provides ``Hitaki-0.0.gir``.

  * https://github.com/alsa-project/libhitaki

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* The latest release is version 0.5.0.

Crates
======

API bindings for safe and high-level abstractions
-------------------------------------------------

* `hitaki crate <hitaki/README.md>`_

`FFI bindings <https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages>`_
--------------------------------------------------------------------------------------------

* `hitaki-sys crate <hitaki-sys/README.md>`_

License
=======

MIT License

Dependencies
============

* `libhitaki <https://github.com/takaswie/libhitaki>`_
* FFI crate (``hitaki-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.20
  * ``gobject-sys`` >= 0.20

* API crate (``hitaki``)

  * ``libc`` >= 0.2
  * ``glib`` >= 0.20
  * FFI crate (``hitaki-sys``)
