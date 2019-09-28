
# Information sur la famille rtlsdr

Le mot RTLSDR décrit une famille de dispositifs USB. Ces engins se basent tous sur le contrôleur realtek 2832u. Il existe plusieurs compagnies qui vendent des configurations différentes. Les variations sont principalement orientés autour du circuit de tuner. Ceci veut dire que la majorité des dispositifs fonctionneront avec l'API open-source (regardez le projet rtlsdr du groupe osmocom). 

Cet API se base sur la librairie LibUsb-1.0. Ceci assure une compatibilité avec la majorité des systèmes Linux sur plusieurs architectures. Cet API est écrit en C. Il est toutefois possible de l'utiliser en C++ et en Rust. 