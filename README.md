# Wiborade

Ce projet, développé en Rust, est une application destinée à la constitution d'une base de données à partir de collections de lettres manuscrites numérisées. Après avoir chargé un dossier contenant les images des lettres, l'utilisateur peut visualiser chaque document, utiliser des fonctions de zoom et de rotation pour en faciliter la lecture, puis renseigner un formulaire décrivant les informations contenues dans le texte de la lettre (personnes, lieux, événements, dates ou tout autre élément d'intérêt). Les données saisies sont automatiquement enregistrées dans un fichier Excel, qui constitue une première étape vers la création d'une base de données unique. L'objectif est de centraliser les informations extraites de plusieurs correspondances, aujourd'hui répartis dans différents dossiers, afin de faciliter leur recherche, leur croisement et leur exploitation au sein d'un même environnement.

## Installation

Comment compiler/lancer le projet :
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Utilisation

Il faut lancer l'exe puis selectionner un dossier contenant des jpg et au moins un excel format xlsx :
```bash
./mon-programme.exe
```

## Fonctionnalités

- Ce que le programme fait
- Point 2
- Point 3

## Statut / Limitations

- Ce qui marche bien
- Ce qui est encore en cours / pas encore géré
- Bugs connus si il y en a

## Changelog

Voir CHANGELOG.md (ou un lien vers les releases GitHub)
