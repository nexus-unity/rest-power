# Power Module RESTful API driver
[![Build](https://github.com/nexus-unity/rest-power/actions/workflows/build.yml/badge.svg)](https://github.com/nexus-unity/rest-power/actions/workflows/build.yml)

This driver provides an HTTP RESTful API and is written in [Rust](https://www.rust-lang.org/) using [Rocket](https://rocket.rs/).  
It uses the [Power Module driver](https://github.com/nexus-unity/drv-power.git) and makes all [Power Module](https://nexus-unity.com/en/modules/power/)
functions available.

The API is stable and should be used for remote as well as local applications.  
The entire documentation can be found [here](https://doc.nexus-unity.com/en/module-restful-api/power-module/).  
The driver currently does not support authentication/session handling. The authentication is done via proxy settings.

## Building
To build this project for the target platform the "armv7-unknown-linux-gnueabihf" target must be installed via *rustup*.    
The "arm-linux-gnueabihf-gcc" linker must also be configured (check the Dockerfile).
```bash
cargo build --target=armv7-unknown-linux-gnueabihf
```
The project can be build directly on the Nexus if Rust is installed, but it will take some time:
```bash
cargo build
```
### Docker
There is a Dockerfile in the project which allows you to build the project for armv7:
```bash
docker build -t rust-cross-build .
docker run -t --rm -u 1000:1000 -w "$PWD" -v "$PWD:$PWD":rw,z rust-cross-build cargo build --target=armv7-unknown-linux-gnueabihf
```

## Executing
The application must be executed on the Nexus and the *nexus-drv-power* service must be running.   
Please ensure the *nexus-rest-power* service is stopped.     
```bash
RUST_APP_LOG="info" ROCKET_ENV=production PORT=8003 ./nexus-rest-power
```

## Packaging
We do not build Debian packages on Github because the armhf architecture is not supported.  
Please check the [packaging guide](https://doc.nexus-unity.com/en/technical-details/packaging/guide/) for details. 

## Testing
The testsuite is currently not published because it relies on hardware, physical test tools and a specific wiring.  

## License
This driver is licensed under [GPLv3](LICENSE).