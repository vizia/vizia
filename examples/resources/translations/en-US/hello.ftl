-brand = Vizia

hello-world = Hello, world!
enter-name = Please enter your name:
intro = Welcome, { $name }.
emails =
    { $unread_emails ->
        [one] You have one unread email.
       *[other] You have { $unread_emails } unread emails.
    }
refresh = Refresh

## Example of message attributes for UI elements
dialog = Dialog
    .title = Confirmation Dialog
    .prompt = Are you sure you want to proceed?
    .confirm-button = Yes
    .cancel-button = No

brand-welcome = Welcome to { -brand }!
