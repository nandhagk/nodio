### Build Instructions

```bash
mkdir build
cd build
cmake ../rtaudio
make

cd ../
export LD_LIBRARY_PATH=./build

g++ main.cpp -Irtaudio -Lbuild -lrtaudio -o ./build/a.out
./build/a.out 2 48000
```
