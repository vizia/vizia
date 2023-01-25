hello-world = Bonjour, monde!
intro = Bienvenue, { $name }.
enter-name = Veuillez saisir votre nom:
emails =
    { $unread_emails ->
        [one] Vous avez { WITH_CLASS("un", "bold") } e-mail non lu.
       *[other] Vous avez { WITH_CLASS($unread_emails, "bold") } e-mails non lus.
    }
refresh = Actualiser la page
