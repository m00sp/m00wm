# m00wm

# Let's write a window manager from scratch

Writing an X11 window manager from scratch using [penrose](https://github.com/sminez/penrose).

You can follow along with the development of this repo on [youtube](https://www.youtube.com/playlist?list=PLy2HjaQiG8lOxCKzuWKfmmXov4iEVOGOF).

This is a work in progress project to build up a fully featured tiling window manager from scratch
so please make sure you have an alternative desktop environment available to use in case anything
breaks!


## Installation

> **NOTE**: Really _do_ read the Makefile before installing: there's nothing harmful in
> there but you should always know what you are running under `sudo`!

Make sure you have [Rust](https://rust-lang.org) installed on your system and take a look
at the default key bindings in `main.rs`, you will want to swap out `st` and `dmenu_run` for
a terminal and program launcher you have installed if you're not using them.

With that done, read the contents of the `Makefile` in the root of the repo before running
the following in a terminal to build and install the window manager:

```sh
$ make build && sudo make install
```

This should set you up for running `penrose-from-scratch` as a desktop session from your
[display manager](https://wiki.archlinux.org/title/Display_manager) when you log in.


## Following along

The [progress-so-far.md](./progress-so-far.md) file in the root of this repository acts as a
bit of a change log and reference for what we've done so far. If you're looking for a summary
of what's been implemented (and when it was implemented) that's a good place to start.

<video controls autoplay loop muted playsinline>
  <source src="imagens/m00alpine2-screen0.webm" type="video/webm" />
  Your browser doesn't support the video tag and/or the video formats in use here – sorry!
</video>


<div align="center">
  <h1 align="center">m00wm</h1>
  <h3>The Rust tiling Window Manager Alternative.</h3>

<a target="_blank" href="https://www.producthunt.com/posts/papermark-3?utm_source=badge-top-post-badge&amp;utm_medium=badge&amp;utm_souce=badge-papermark"><img src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=411605&amp;theme=light&amp;period=daily" alt="Papermark - The open-source DocSend alternative | Product Hunt" style="width:250px;height:40px"></a>

</div>

<div align="center">
  <a href="https://www.luis-ti.dev.br">www.luis-ti.dev.br</a>
</div>

<br/>

<div align="center">
  <a href="https://github.com/mfts/papermark/stargazers"><img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/mfts/papermark"></a>
  <a href="https://twitter.com/papermarkio"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/papermarkio"></a>
  <a href="https://github.com/mfts/papermark/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/license-AGPLv3-purple"></a>
</div>

<br/>

Papermark is the open-source document sharing alternative to DocSend with built-in analytics and custom domains.

## Features

- **Shareable Links:** Share your document securely by sending a custom link
- **Custom Branding:** Add a custom domain and your own branding
- **Analytics:** Get insights via document tracking and soon page-by-page analytics
- **Self-hosted, open-source:** Host it yourself and hack on it

## Demo

![Papermark Welcome GIF](.github/images/papermark-welcome.gif)

## Tech Stack

- [Next.js](https://nextjs.org/) – Framework
- [Typescript](https://www.typescriptlang.org/) – Language
- [Tailwind](https://tailwindcss.com/) – CSS
- [shadcn/ui](https://ui.shadcn.com) - UI Components
- [Prisma](https://prisma.io) - ORM [![Made with Prisma](https://made-with.prisma.io/dark.svg)](https://prisma.io)
- PostgreSQL - Database
- [NextAuth.js](https://next-auth.js.org/) – Authentication
- [Tinybird](https://tinybird.co) – Analytics
- [Resend](https://resend.com) – Email
- [Stripe](https://stripe.com) – Payments
- [Vercel](https://vercel.com/) – Hosting

## Getting Started

### Prerequisites

Here's what you need to be able to run Papermark:

- Node.js (version >= 18)
- PostgreSQL Database
- Blob storage (currently [AWS S3](https://aws.amazon.com/s3/) or [Vercel Blob](https://vercel.com/storage/blob))
- [Resend](https://resend.com) (for sending emails)

### 1. Clone the repository

```shell
git clone https://github.com/mfts/papermark.git
cd papermark
```

### 2. Install npm dependencies

```shell
npm install
```

### 3. Copy the environment variables to `.env` and change the values

```shell
cp .env.example .env
```

### 4. Initialize the database

```shell
npx prisma generate
npx prisma migrate deploy
```

### 5. Run the dev server

```shell
npm run dev
```

### 6. Open the app in your browser

Visit [http://localhost:3000](http://localhost:3000) in your browser.

## Tinybird instructions

To prepare the Tinybird database, follow these steps:

0. We use `pipenv` to manage my Python dependencies. If you don't have it installed, you can install it using the following command:
   ```sh
   pkgx pipenv
   ```
1. Download the Tinybird CLI from [here](https://www.tinybird.co/docs/cli.html) and install it on your system.
2. After authenticating with the Tinybird CLI, navigate to the `lib/tinybird` directory:
   ```sh
   cd lib/tinybird
   ```
3. Push the necessary datasources using the following command:
   ```sh
   tb push datasources/*
   tb push endpoints/get_*
   ```
4. Don't forget to set the `TINYBIRD_TOKEN` with the appropriate rights in your `.env` file.

#### Updating Tinybird

```sh
pipenv shell
## start: pkgx-specific
cd ..
cd papermark
## end: pkgx-specific
pipenv update tinybird-cli
```

## Contributing

Papermark is an open-source project and we welcome contributions from the community.

If you'd like to contribute, please fork the repository and make changes as you'd like. Pull requests are warmly welcome.

### Our Contributors ✨

<a href="https://github.com/m00sp/m00wm/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=m00sp/m00wm" />
</a>

