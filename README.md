# StarryNight
Paint your pallet blue and gray, 

Look out on a summers day,

With eyes that know the darknes in my soul.

![image](img/vincent.png)

## Prerequisites
- [Rust](https://www.rust-lang.org/)
- [OpenCV](https://github.com/opencv/opencv)

### OpenCV Setup Guide (Linux)
There are a couple different ways to install OpenCV but I typically just clone the latest repo and build it from source.

Once you clone the repo, here is an outline of the steps I usually follow
```
cmake -B build -S . -DCMAKE_INSTALL_PREFIX=/usr/local -DOPENCV_GENERATE_PKGCONFIG=ON -DWITH_EIGEN=OFF ..

cd build && make -j8 && sudo make install
```


### Build 
```
git clone https://github.com/sjbeskur/starrynight.git
cd starrynight
cargo build
```

### Run
Once the cargo build completes all build artifacts will be placed in the target directory under debug or release depending on the flag you set above.

To run the application simply:
```
./target/release/starrynight <path to starts>
```

e.g.
```
./target/release/starrynight ./img/71_stars.png
```

### Help:

![help](img/help.png)