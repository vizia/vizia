-app-name = Vizia

system-theme = Système
dark-theme = Sombre
light-theme = Clair
blue = Bleu
emerald = Émeraude
crimson = Cramoisi
amber = Ambre
violet = Violet
en = Anglais
fr = Français
ar = Arabe
button = Bouton
secondary-button = Bouton secondaire
outline-button = Bouton contour
text-button = Bouton texte
button-with-icon = Bouton avec icône
checkbox = Case à cocher
one = Un
two = Deux
three = Trois
toggle-disabled = Basculement Désactivé
default = Défaut

## Example message with attributes
form-submission = Soumission du formulaire
    .label = Soumettre le formulaire
    .help-text = Tous les champs sont requis
    .success-message = Formulaire soumis avec succès

## Number formatting examples
item-count = Vous avez { $count } éléments
discount-percent = Remise: { $discount }%

## Date formatting examples
joined-date = Membre depuis { DATETIME($date, year: "numeric", month: "long", day: "numeric") }
last-login = Dernière connexion: { DATETIME($date, month: "short", day: "numeric", hour: "2-digit", minute: "2-digit") }

## Calendar
calendar-previous-month = Mois précédent
calendar-next-month = Mois suivant
calendar-week-start = monday
calendar-month-year-heading = { DATETIME($date, year: "numeric", month: "long") }
calendar-day-cell-name = { DATETIME($date, weekday: "long", year: "numeric", month: "long", day: "numeric") }
calendar-keyboard-help = Utilisez les flèches pour les jours, Début/Fin pour les limites de semaine, Page précédente/Page suivante pour le mois, Maj plus Page précédente/Page suivante pour l'année, et Entrée ou Espace pour sélectionner.

Jan = Janv
Feb = Févr
Mar = Mars
Apr = Avr
May = Mai
Jun = Juin
Jul = Juil
Aug = Août
Sept = Sept
Oct = Oct
Nov = Nov
Dec = Déc

Monday = Lundi
Tuesday = Mardi
Wednesday = Mercredi
Thursday = Jeudi
Friday = Vendredi
Saturday = Samedi
Sunday = Dimanche

Monday-short = Lu
Tuesday-short = Ma
Wednesday-short = Me
Thursday-short = Je
Friday-short = Ve
Saturday-short = Sa
Sunday-short = Di

## Accordion
accordion-title-1 = Aperçu du projet
accordion-content-1 = Vizia est un framework GUI déclaratif pour les applications de bureau.
accordion-title-2 = Installation
accordion-content-2 = Ajoutez `vizia` à vos dépendances et exécutez l'application.
accordion-title-3 = Style
accordion-content-3 = Utilisez des feuilles de style de type CSS et des sélecteurs de classe pour personnaliser votre interface utilisateur.
allow-multiple-open = Autoriser plusieurs ouverts
toggle-section = Basculer la deuxième section

## View example window titles
view-title-textbox = Champ de texte
view-title-scrollview = Vue défilante
view-title-progressbar = Barre de progression
view-title-label = Étiquette
view-title-menubar = Barre de menus
view-title-button-group = Groupe de boutons
view-title-vstack = Pile verticale
view-title-svg = SVG
view-title-collapsible = Panneau repliable
view-title-resizable = Redimensionnable
view-title-xypad = Pavé XY
view-title-radiobutton = Bouton radio
view-title-card = Carte
view-title-tabview = Vue à onglets
view-title-virtual-table = Table virtuelle de grand jeu de données
view-title-tooltip = Info-bulle
view-title-knob = Molette
view-title-toggle-button = Bouton bascule
view-title-markdown = Markdown
view-title-menu = Menu
view-title-list = Liste
view-title-divider = Séparateur
view-title-accordion = Accordéon
view-title-hstack = Pile horizontale
view-title-slider = Curseur
view-title-spinbox = Sélecteur numérique
view-title-dropdown = Liste déroulante
view-title-calendar = Calendrier
view-title-avatar = Image de profil
view-title-table = Tableau
view-title-chip = Puce
view-title-select = Sélection
view-title-virtual-list = Liste virtuelle
view-title-switch = Switch
view-title-zstack = Empilement Z
view-title-combobox = Zone combinée
view-title-rating = Évaluation

## Textbox and scrolling
textbox-placeholder-type = Tapez quelque chose...
textbox-placeholder-search = Rechercher
scroll-vertical = Défilement vertical
scroll-horizontal = Défilement horizontal
scroll-horizontal-vertical = Défilement horizontal et vertical

## Label, checkbox, switch, and select examples
label-static-unicode = Une étiquette peut afficher une chaîne Unicode statique
label-wrap-enabled = Le texte trop long pour l'étiquette sera renvoyé à la ligne.
label-wrap-disabled = Sauf si le retour à la ligne est désactivé.
label-describing-trigger = Une étiquette décrivant un élément de formulaire agit aussi comme déclencheur
checkbox-with-label = Case à cocher avec étiquette
checkbox-with-custom-icon-label = Case à cocher avec icône personnalisée et étiquette
switch-basic = Commutateurs de base
switch-1 = Commutateur 1
switch-2 = Commutateur 2
select-placeholder = Sélectionnez une option...

## Button group, card, and collapsible examples
button-accept = Accepter
button-maybe = Peut-être
button-decline = Refuser
button-top = Haut
button-middle = Milieu
button-bottom = Bas
card-starter-title = Offre Starter
card-starter-description = Pour les prototypes et les expériences rapides
card-starter-price = 9 $ / mois
card-starter-feature-1 = Jusqu'à 3 projets
card-starter-feature-2 = Support communautaire
card-starter-feature-3 = Espaces de travail partagés
card-choose-plan = Choisir l'offre
card-team-title = Offre Équipe
card-team-description = Plus de contrôle pour les applications en production
card-team-price = 29 $ / mois
card-team-feature-1 = Projets illimités
card-team-feature-2 = Support prioritaire
card-team-feature-3 = Personnalisation du thème
card-preview = Aperçu
card-upgrade = Mettre à niveau
collapsible-toggle = Basculer l'état replié
collapsible-header = Cliquez pour développer ou réduire cette section
collapsible-content-long = Ce panneau contient un bloc de contenu plus long. Il illustre le composant repliable avec plusieurs lignes de texte pouvant être affichées ou masquées en cliquant sur l'en-tête ci-dessus.
collapsible-content-short = Ce panneau contient un court bloc de contenu qui peut être affiché ou masqué.
action-cancel = ANNULER
action-save = ENREGISTRER

## Menu and menubar examples
menu-root = Menu
menu-new = Nouveau
menu-shortcut-new = Ctrl + N
menu-open = Ouvrir
menu-shortcut-open = Ctrl + O
menu-open-recent = Ouvrir récent
menu-doc-1 = Doc 1
menu-doc-2 = Doc 2
menu-version-1 = Version 1
menu-version-2 = Version 2
menu-version-3 = Version 3
menu-doc-3 = Doc 3
menu-save = Enregistrer
menu-save-as = Enregistrer sous
menu-quit = Quitter
menubar-file = Fichier
menubar-edit = Édition
menubar-view = Affichage
menubar-help = Aide
menubar-cut = Couper
menubar-copy = Copier
menubar-paste = Coller
menubar-zoom-in = Zoom avant
menubar-zoom-out = Zoom arrière
menubar-zoom-level = Niveau de zoom
menubar-zoom-10 = 10%
menubar-zoom-20 = 20%
menubar-zoom-50 = 50%
menubar-zoom-100 = 100%
menubar-zoom-150 = 150%
menubar-zoom-200 = 200%
menubar-show-license = Afficher la licence
menubar-about = À propos

## Tooltip and table examples
tooltip-placement-top-start = Haut début
tooltip-placement-top = Haut
tooltip-placement-top-end = Haut fin
tooltip-placement-left-start = Gauche début
tooltip-placement-left = Gauche
tooltip-placement-left-end = Gauche fin
tooltip-placement-right-start = Droite début
tooltip-placement-right = Droite
tooltip-placement-right-end = Droite fin
tooltip-placement-bottom-start = Bas début
tooltip-placement-bottom = Bas
tooltip-placement-bottom-end = Bas fin
tooltip-placement-over = Sur
tooltip-placement-cursor = Curseur
tooltip-text = Ceci est une info-bulle
table-show-group = Afficher le groupe
table-show-notes = Afficher les notes
table-prioritize-status = Prioriser le statut

## XY pad, radio button, and virtual table examples
xypad-heading = XY Pad bidimensionnel
radiobutton-basic = Boutons radio de base
radiobutton-with-labels = Boutons radio avec étiquettes
option-first = Premier
option-second = Deuxième
option-third = Troisième
toggle-bold = Gras
virtual-table-heading = VirtualTable grand jeu de données (5 000 lignes)

## Chip example
chip-label = Puce
chip-color-red = rouge
chip-color-green = vert
chip-color-blue = bleu
virtual-table-description = Lignes virtualisées à hauteur fixe pour les grands jeux de données. Utilisez le tri et le redimensionnement pour analyser l'interactivité.

## Select example
select-option-one = Un
select-option-two = Deux
select-option-three = Trois
select-option-four = Quatre
select-option-five = Cinq
select-option-six = Six vraiment long
select-option-seven = Sept
select-option-eight = Huit
select-option-nine = Neuf
select-option-ten = Dix
select-option-eleven = Onze
select-option-twelve = Douze