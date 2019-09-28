\pagebreak

# Itérations

Le projet devra suivre quelques étapes importantes. On peut s'imaginer le projet en suivant le modèle des couches OSI. 

* La version 0.1 représente la couche physique, liaison et réseau.
* La version 0.2 représente la couche transport.
* La version 0.3 représente la couche application.

## Version 0.1

La première itération consiste à obtenir le matériel et à être capable de l'utiliser. Il faut donc obtenir un module RTLSDR et être capable de le contrôler. On peut obtenir plusieurs de ces modules pour une vingtaine de dollars sur Amazon. Tant qu'au contrôle, il suffit de produire un petit programme capable de contrôller des paramètres du module et obtenir des données de celui-ci. 

Pour prouver que le programme fonctionne correctement, il graphera les samples obtenus auprès du module USB. De plus, il suffiera de faire rouler le programme de test sur un PC.

Selon ce [site web](https://www.sigidwiki.com/wiki/Automatic_Picture_Transmission_(APT)#Images), le signal reçu est un signal AM qui fût modulé dans un signal FM. Il faut donc tenir compte de cela dans la démodulation. De plus, le module USB capte des samples de tension en bits.

\begin{equation}
V_(signal) = A*sin(2*\pi*F(t) )
\end{equation}
Dans cette équation, F(t) est 

## Version 0.2

La deuxième itération du projet se concentre sur la démodulation. En suivant la spécification APT qu'utilise les satellites de l'agence NOAA, on doit convertir les signaux radio en série d'octets. Pour se faire, il faut mettre la main sur le standard APT. Évidemment il y a plus de recherches à faire sur le sujet.

Concernant la plateforme, cette version peut aussi se contenter de fonctionner sur un PC.


![Image montrant le standard APT (https://en.wikipedia.org/wiki/Automatic_picture_transmission)](Doc/NOAA_APT_Frame_Format-Wikipedia.png)

\pagebreak
## Version 0.3
Avec cette version, le programme doit être capable de convertir les octets reçus en format d'image. 