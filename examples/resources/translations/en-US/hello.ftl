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

## Example of message references
menu-save = Save
help-menu-save = Click { menu-save } to save the file.

## Example of message attributes for UI elements
dialog = Dialog
    .title = Confirmation Dialog
    .prompt = Are you sure you want to proceed?
    .confirm-button = Yes
    .cancel-button = No

brand-welcome = Welcome to { -brand }!

## Number formatting example
price = Price: { NUMBER($amount) }
percentage-complete = { $percent }% complete

## Date formatting example
event-date = Event date: { DATETIME($date, weekday: "long", month: "long", day: "numeric", year: "numeric") }
last-updated = Last updated: { DATETIME($date, month: "short", day: "numeric", year: "2-digit", hour: "numeric", minute: "2-digit") }
