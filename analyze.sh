#!/bin/bash
which tokei > /dev/null || exit
cd aosp15 || exit

rm -rf ../stats/
mkdir ../stats

# Classified directories: 
# - https://xdaforums.com/t/guide-understanding-the-android-source-code.2620389/
# - https://stackoverflow.com/questions/9046572/how-to-understand-the-directory-structure-of-android-root-tree/9047693#9047693
tokei art/ bionic/ bootable/ dalvik/ frameworks/ hardware/ libcore/ libnativehelper/ system/ --output json > ../stats/core.json
tokei packages/ --output json > ../stats/userspace.json
tokei external/ kernel/ --output json > ../stats/thirdparty.json
tokei build/ cts/ tools/ pdk/ platform_testing/ sdk/ test/ toolchain/ --output json > ../stats/devtools.json
tokei development/ developers/ --output json > ../stats/sdks.json
# ignore: device/ prebuilts/
# What does: trusty/