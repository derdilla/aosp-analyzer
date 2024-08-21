#!/bin/bash

mkdir aosp15
cd aosp15 || exit
repo init -u https://android.googlesource.com/platform/manifest -b main
repo sync -c -j8
