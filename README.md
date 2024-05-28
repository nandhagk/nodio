### Build Instructions

```bash
git submodule update --init --recursive

mkdir build
cd build
FOR NON BAKED:
    cmake ../
FOR WINDOWS:
    cmake ../ -DCMAKE_CXX_COMPILER="g++.exe" -G "MinGW Makefiles"
make
./nodio
```
