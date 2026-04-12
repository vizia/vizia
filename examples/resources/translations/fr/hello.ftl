-brand = Vizia

hello-world = Bonjour, monde!
intro = Bienvenue, { $name }.
enter-name = Veuillez saisir votre nom:
emails =
    { $unread_emails ->
        [one] Vous avez un e-mail non lu.
       *[other] Vous avez { $unread_emails } e-mails non lus.
    }
refresh = Actualiser la page

## Example of message attributes for UI elements
dialog = Dialogue
    .title = Dialogue de Confirmation
    .prompt = Êtes-vous sûr de vouloir continuer?
    .confirm-button = Oui
    .cancel-button = Non

brand-welcome = Bienvenue à { -brand }!

## Number formatting example
price = Prix: { NUMBER($amount) }
percentage-complete = { $percent }% complété

## Date formatting example
event-date = Date de l'événement: { DATETIME($date, weekday: "long", month: "long", day: "numeric", year: "numeric") }
last-updated = Dernière mise à jour: { DATETIME($date, month: "short", day: "numeric", year: "2-digit", hour: "numeric", minute: "2-digit") }