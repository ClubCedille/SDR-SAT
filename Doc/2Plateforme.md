
# Plateformes d'intérêts potentiels

Par plateforme, on entend médium d'installation. La première plateforme me venant à l'esprit est le Raspberry Pi. Mais, il est absolument possible de faire un programme de décodage et de démodulation sur un PC. De plus, tant que nous avons accès à la librairie rtlsdr sous une forme ou l'autre il n'y a pas de limites. La plateforme choisie est donc très peu importante. 

Toutefois, je ne recommande pas l'utilisation de microcontrolleurs (Arduino). Leurs basses vitesses de calculs est un facteur extrêmement limitant. De plus, la majorité des micro-contrôleurs auront besoin d'un système extèrne de conversion USB->Port sériel.
Finalement, ils n'ont pas de capacités de multi-threading (ne pas confondre avec des intérruptions). Ceci est un gros obstacle si on considère le besoin de futur de communiquation internet.