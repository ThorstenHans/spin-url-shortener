# URL Shortener

The _URL Shortener_ is a fairly simple app to demonstrate and teach the fundamentals of Fermyon Spin. 

The app consists of two Spin components

- Frontend (static file server) for serving the web app
- Backend (Rust) a set of APIs required for the _URL Shortener_

## How to run the app locally

To run the app locally, you must have `spin` CLI installed on your machine. Clone the repo and navigate to the root folder of the project. From here you can use `spin` CLI to build and run the app:

```bash
# Build the app
spin build

# Run the app
spin up --direct-mounts
```

By providing `--direct-mounts`, the frontend will pick up modifications to the frontend, without having to restart the app itself.

## How to deploy the app to Fermyon Cloud

You can deploy the app to Fermyon Cloud using `spin` CLI. First, you have to authenticate `spin` CLI with your Fermyon Cloud account:

```bash
# Authenticate with Fermyon Cloud
spin cloud login

# Deploy the app
spin cloud deploy
```
