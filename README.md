<div style="text-align: center;">
<h1>
   🔗 Workers link shortener 🔗
</h1>
</div>

This is a dead-simple link shortener written
in [Rust](https://www.rust-lang.org/) that runs
on [Cloudflare Workers](https://workers.cloudflare.com/)
and uses [KV](https://developers.cloudflare.com/workers/runtime-apis/kv) to store shotened links.

## Prerequisites

You'll want [`npx`](https://www.npmjs.com/package/npx) to run the
`wrangler` command. I put the most relevant commands into a `Justfile`
so you can run them with [just](https://github.com/casey/just).
Clearly, you might want to install `just`. Otherwise, you can run
the commands directly. If you encounter other challenges, please
read [these instructions on Cloudflare](https://developers.cloudflare.com/workers/languages/rust/).

## Clone the repository

I assume you can do that.

## Customize the configuration

You'll need to [create a KV
namespace](https://developers.cloudflare.com/kv/get-started/#2-create-a-kv-namespace).
To do that run `just create-kv-namespace`. If you
get an error related to permissions, you'll need to login with `just
login`

Generating the KV namespace will return an ID and a name. You'll need
to update `wrangler.toml` with the ID. There should be
a section that looks like this.

```toml
[[kv_namespaces]]
binding = "workers_link_shortener"
id = "YOUR_ID_GOES_HERE"
```

Of course, you can change the worker name and also the binding 
of the KV namespace. The worker name is set in `wrangler.toml` and
the binding appears in `wrangler.toml`, `Justfile`, and `src/lib.rs`. 

## Running locally

To run locally, do `just run-dev`. This will start [miniflare](https://miniflare.dev/).
To add a redirect to the local KV store, you can run `just add-local-redirect SLUG URL`,
e.g. `just add-local-redirect g https://google.com`.

## Deploying

Run `just deploy` to deploy the worker. This will build the worker and
deploy it to Cloudflare. It will return a URL to you like
 <https://workers-link-shortener.some-account-name.workers.dev>.

 To add redirects in production, run `just add-redirect SLUG URL`, e.g. `just add-redirect g https://google.com`. Right now the only way to add redirects is from
 the command line. I might add an API at some point.


## Adding a custom domain

Once your worker is running, you can add a custom domain. See the
[Cloudflare documentation on custom domains](https://developers.cloudflare.com/workers/configuration/routing/custom-domains/). And there you have it,
your own link shortener.

## Deleting a redirect


