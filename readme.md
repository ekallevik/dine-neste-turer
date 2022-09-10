# Dine Neste Turer - An unofficial DNT activity notification service

DNT is the Norwegian Trekking Association, and the host a wide variety of outdoor events and trips. These are extremly popular in Norway, but unfortunately there is no way to get notified whenever new activities are added. Dine Neste Turer provides the user with a notification service for relevant activities by scraping this website.

DNT lists all their activities at dnt.no/aktiviteter, and it is possible to create a user-defined filter. The filter values are stored in the URL, and consequently makes an ideal basis for a scraping service. Dine Neste Turer will periodically search for any new activities through the provided URL filter, and notify the user of the new matches.

Note: This is a hobby project, meant for non-profit and personal use. Use common sense if using this code.
Run Locally

Clone the project
```bash
  git clone https://github.com/ekallevik/dine-neste-turer
```

Go to the project directory
```bash
  cargo run
```
