### Build Instructions

```bash
git submodule update --init --recursive

mkdir build
cd build
FOR NON BAKED:
    cmake ../rtaudio
FOR WINDOWS:
    cmake ..\rtaudio -DCMAKE_CXX_COMPILER="g++.exe" -G "MinGW Makefiles"
make

cd ../
export LD_LIBRARY_PATH=./build

g++ main.cpp -Irtaudio -Lbuild -lrtaudio -o ./build/a.out
./build/a.out 2 48000
```
