# SDR-SAT

# Dependencies
* Dependant on the crate rtlsdr v0.1.4
* Dependant on the C lib "rtlsdr" & "rtlsdr-dev". That library is available from the [osmocom organisation](https://github.com/osmocom/rtl-sdr)

# What ?
This project has a goal to pull images from the weather satellites via the ATP protocol. 

# Why ? 
The hobbyist community seems to rely on a closed source & out of date software. Also, they rely on a key-generator to activate the license. This tiny quirk exists because the company the maintained the software no longer exists...

# Necessary hardware
* Any rtlsdr usb dongle.
* A raspberry Pi or a PC.

# Target plateform
* Raspberry/Linux
* X86-64/Linux

Libs are available out there.

# Building the project:
* Building with "cargo build" will only build an executable from the code.
* If you desire to build the documentation, build with "make all"

# How to contribute

* Clone the repo.

* Do your changes. Document them.

* Simply hit me up with a pull request. Justify it by linking with any thread in the issues section. If there aren't any create one !

* If you are a member of the Cédille club, I assume we talked about that pull request beforehand and there shouldn't be any problem.

* Be sure to do a rustfmt before pull request. 

