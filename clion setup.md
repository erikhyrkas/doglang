Install CLion

NOTE: CLion has a free 30 day trial, but you will need a license to use longer. I believe it is possible to use
Microsoft Visual Code for free, but I have not done this. I believe that it doesn't have the same native debug
capabilities as using CLion. It is also possible to use the Rust plugin with Intellij IDEA professional (I don't think
that it works with the community edition), but it also doesn't have the same native debug capabilities as CLion. It is
hard to say whether you'd actually need such debug capabilities, but I didn't want to risk it.

* Download and run: https://www.jetbrains.com/clion/download/

* Add Toolchain

    * Open Settings > Build, Execution, Deployment > Toolchains

    * Click the `+`

    * Select `Visual Studio`

    * Verify that it is detected properly

* Install plugin for Rust

Install the IntelliJ Rust plugin either directly from the plugin repository or right from CLion: go to Settings /
Preferences | Plugins, switch to Marketplace, and type Rust in the search field, then click Install in the plugin
description dialog.

NOTE: I had to close and restart CLion after installing the plugin with all projects closed.
