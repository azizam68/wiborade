## [0.1.0] - 2026-07-07
### Added
- Version initiale envoyée à Floriana Ceresato

## [0.1.1] - 2026-07-08
### Changed
- Découpage de `gui.rs` en modules dédiés : `gui/update.rs`, `gui/view.rs`, `excel/reader.rs`, `excel/writer.rs`, `image_ops.rs`, `files.rs`, `message.rs`. Aucun changement de logique métier.

### Fixed
- `current_dir` par défaut : suppression d'un `.unwrap()` qui pouvait faire paniquer l'app au démarrage si le dossier home n'était pas détectable. L'app démarre maintenant avec `current_dir: None` dans ce cas au lieu de crasher.
- `ExcelSubmit` : suppression d'un `.unwrap()` qui pouvait paniquer si l'index de ligne courant sortait des bornes du tableau excel. Remplacé par une gestion sûre du cas manquant.

## [0.1.2] - 2026-07-08

### Fixed
- Lors de l'update, la ligne du fichier Excel (1..) servait d'index pour un vecteur (0..) donc sur un update la photo suivante semblait conserver les infos de la precedente.

## [0.1.3] - 2026-07-09
### Changed
- Ajout de `gui/subscription.rs` dans `gui.rs` pour permettre l'utilisation des touches tab, fleche haut et bas dans le form

## [0.1.4] - 2026-07-10
### Changed
- Ajout de la barre de status
- Ajout du zoom et rotation de l'image

## [0.1.5] - 2026-07-12
### Changed
- [v0.1.4.1] Les fichiers excels vide ne font plus planter l'application car on verifie que le nombre de colonne est au moins egale au nombre necessaire en dur pour afficher le formulaire
- Corrige le comportement de l'app quand on utilise un clavier numerique
- Le fichier de listing est automatiquement crée si absent

## [0.1.6] - 2026-07-23
### Changed
- fix le bug quand on annule le selecteur de dossier alors qu'il y a deja un dossier en cours
- ajoute une bordure orange sur l'edit input
- retire les messages dans la console

## [0.1.7] - 2026-08-03
### Changed
- Le nom de l'application devient 'Wiborade'