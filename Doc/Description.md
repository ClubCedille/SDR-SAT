# Décodage d'Images satellites:
Proposition par: Vincent Perrier
24 septembre 2019 @ 00:52

## Pourquoi ?
Des satellites passent au dessus de vos têtes tous les jours. Ces satellites ont le mandat de faire une diffusion d'images haute résolution de la surface et de l'atmosphère planétaire. Ces images sont utilisé par les météorologistes comme information pour leurs analyses du climat. Des sytèmes d'alertes précoces aux sinistres dépendent de ces photos. Environment Canada utilise des logiciels avancés et propriétaires pour obtenir ces données. 

Une partie importante du système est le décodage. Certaines personnes le font avec Python et GNU-Radio. Le problème avec cette méthode est le temps réel. GNU radio ne supporte que l'analyse de fichiers de dompage. Une autre manière est l'utilisation de WXtolmg. C'est un logiciel propriétaire daté qui est utilisable seulement après avoir appliqué un key-gen. C'est parce que la compagnie n'existe plus...

Bref, c'est une infrastructure absolument fantastique en manque désespéré d'open-source. C'est aussi une merveilleuse introduction au monde du Software Defined Radio (SDR). Ce monde contient les radios amateurs et les télécommunications sans-fil. Si vous aimez le open-source et l'idée de descendre plus près du matériel, ce projet est pour vous !

## Comment ?

Les satellites météos se nomment NOAA. Il y en a présentement 3 en opération, en ce moment même. Ils diffusent constamment un signal FM décrivant les images. Ces diffusions sont non encryptés et libre à tous d'intercepter. Selon moi le projet se déroulerai un peu comme ceci:

* Acquérir un module RTL-SDR. (disponible sur amazon pour ~20$). 
* Acquérir une plateforme de traitement. E.g Raspberry Pi. Des ports USB sont essentiels.
* Avec l'API open-source rtl-sdr, créer un démodulateur FM. L'API est disponible en plusieurs languages comme C/C++, rust etc...
* Une fois que nous avons trouvé une manière fiable de convertir les ondes FM en séries de bits, il faut implémenter le protocole ATP. Ce protocol est la recette de convertion entre une séries de bits et un fichier d'image. 
* Créer une architecture de site web pouvant télécharger les photos capturés de plusieures stations ou postes d'écoutes.
* Implémentation d'un service d'analyse de photo et de prédiction météorologiques
* Adapter l'architecture pour pouvoir recevoir toutes sortes de transmissions radio à partir de plusieurs stations différentes.

# Buts
Même si nous regardons le cosmos avec envie, il faut garder les pieds au sol.

## But initial
Le premier objectif est de pouvoir obtenir des images à l'aide de logiciel open-source et temps réel. Par temps-réel, on entend un modèle de streaming vers un serveur central. Ceci est différent de l'approche de GNU-Radio. GNU-radio fait des écritures répétitives dans un fichier donné. De manière évidente, cela peut apporter des problèmes de performances.

Par intérêt personnel envers le language, j'aimerais pouvoir implémenter le démodulateur FM et le décodeur ATP en language Rust. C'est bien supporté sur les Raspberry Pi de divers générations et assez performant pour nos besoins. De plus, cela donnerais une bonne raison aux participants de se mettre à apprendre le language plus sérieusement.

## Améliorations futures
Dans un monde idéal, les divers étapes de traitement de donnée seraient indépendantes. On pourrait avoir des stations d'écoutes servant uniquement à streamer les données à une unité centrale. Cette unité aurait alors comme mandat de faire le décodage. Pour l'instant ce n'est rien de concret, plus à suivre et plus à discuter dans le futur.

# À l'ordre du jour lors de la rencontre de mercedi 25 septembre 2019:
Voici quelques questions que j'aimerais vous poser par rapport au projet:
* Est-ce une bonne idée ?
* Est-ce que ce projet semble aligné avec le but du club ?
* Le but initial vous semble-t-il réaliste ?
* Aimez-vous la perspective d'analyse d'image après l'implémentation de l'infrastructure ?
* Pensez-vous que le AI pourrait être introduit là-dedans ?