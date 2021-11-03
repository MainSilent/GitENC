# GitENC

Encrypt and Decrypt all your repository data.

I sometimes have very sensitive codes that even making the repository private isn't enough, I want to make sure if someone gain access to the repo they can't read the code as a clear text.

So I decided to work on this project and also get more experience with rust.

Note: There are other projects with the same name or purpose but they don't have the requirements that I need, I need a simple command to compress and encrypt the data and when clonning do the reverse.

BTW this project is my first experience with rust and only linux is supported.

---

## Build

```
cargo build
```

## Setup

Make sure you have `openssl` and `git` installed

Create a repo and set `GITENC_PASSWORD` in .env for password

### .gitignore
```
*
!.gitignore
!data.enc
```

## How to Commit
Run the following command to compress, encrypt and commit to git
```
gitenc commit
```

## How to Clone
Create an empty directory and setup the .env and run the command below
```
gitenc clone https://{username}:{password}@github.com/{owner}/{repo}
```
