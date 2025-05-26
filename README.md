# JukenSwitecStepperMotors-rs
A Rust-based embedded (no-std) driver for direct driving the [Juken-Swiss Tech X2x gauge stepper motors](https://jukenswisstech.com/project/stepper-motors/), like the X25 and X27.

# Hi Guy
This is a loose rewrite/fork/inspired by Guy Carpenter's excellent [SwitecX25 Arduino library](https://github.com/clearwater/SwitecX25). Guy also has a terrific amount of info on these steppers on [their website](https://guy.carpenter.id.au/gaugette/resources/) to support their [gaugette](https://github.com/clearwater/gaugette) and [py-gaugette](https://github.com/guyc/py-gaugette) projects. This work seems to be unmaintained now but credit is owed regardless.

# The Motors
These are original equipment in many automotive OEM dashboards, making them cheap and readily available. They've become popular for hobbyists developing simrigs (both automotive and aircraft) and retrofitting vintage gauge clusters to modern technology. There is also a part number (X10) for analog clocks that allows continuous rotation, though most of the cheap X25 and X27 motors only allow about 315 deg of rotation. The motors were originally known only as Switec (Swiss-Tech) but it looks like that company merged with or was acquired by Juken and they're now sold under both names.

There are other motors that are obvious Juken-Switec clones, like the VID29 and MCR1108. This crate should be compatible with those.

# The Driver
This is a no-std Rust crate that will utilize `embedded-hal` traits and offer both non-blocking/asynchronous `embassy` based functions as well as blocking functions. It's an active work-in-progress and is not in a stable state (yet).
