hello-world = Hello, world!
enter-name = Please enter your name:
intro = Welcome, { $name }.
emails =
    { $unread_emails ->
        [one] You have { WITH_CLASS("one", "bold") } unread email.
       *[other] You have { WITH_CLASS($unread_emails, "bold") } unread emails.
    }
refresh = Refresh
