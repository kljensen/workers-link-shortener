
binding:="workers_link_shortener"

default:
  @just --list

run-dev:
    npx wrangler dev

add-redirect slug url:
    npx wrangler kv:key put --binding=$binding {{slug}} {{url}}

add-local-redirect slug url:
    npx wrangler kv:key put --binding=$binding {{slug}} {{url}} --local

create-kv-namespace:
    npx wrangler kv:namespace create {{binding}}

login:
    npx wrangler login

deploy:
    npx wrangler deploy

