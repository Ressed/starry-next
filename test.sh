make defconfig ARCH=x86_64 EXTRA_CONFIG=../configs/x86_64.toml
cp ./sdcard-x86_64.img .arceos/disk.img
# cp ./disk.img .arceos/disk.img
make AX_TESTCASE=junior ARCH=x86_64 EXTRA_CONFIG=../configs/x86_64.toml BLK=y NET=y FEATURES=fp_simd,lwext4_rs SMP=1 LOG=off run