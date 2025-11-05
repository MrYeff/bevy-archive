# Architecture

The application itself shouldnt store files, this is offloadded to github.
This is the draft for the first stage of development. This only includes registering and displaying posts and meta information. No user identification.

## Technology

### Frontend

Js should be minimized. The frontend stack is:

Htmx > axum_template > terra

### Core

webserver: axum
db connector: sqlx
outgoing calls: reqwest
chache: redis

### Md Conversion

comrak

## Incomming Calls

### Main

#### fetch:/ -> htmx

return front page

### Reader

#### fetch:/articles/(id) -> htmx::Page

Intent: Load the metadata of an article

- get meta from db (tags, name, author, likes, post ...)
- build htmx using terra
- return

#### fetch:/articles/content/(id) -> htmx::Page

Intent: Load the post content

- if in chache: return chached
- else:
  - request build
  - return "retry later"

### Register

#### fetch:/articles/new -> htmx::Page

return register page (1 box for the github url)

#### fetch:/articles/new/load(url) -> htmx::Page

- if in chache:
  - get meta from chache
  - build htmx using terra
  - return
- else:
  - request load
  - return "retry later"

#### post:/articles/new/register(url) -> htmx::Page

expects: meta load to chache

- take from chache
- push to db

## Workers

### Build Article

- loop:
  - if let id = request build article(id)
    - fetch md from github
    - render to html
    - inject reference snipets
    - write to chache

### Load New Article

- loop:
  - if let url = request load article(url)
    - fetch archive.toml from github
    - write to chache
