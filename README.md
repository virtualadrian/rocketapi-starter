# rocketapi-starter

### Rust and Rocket ... ~ this is the aftermath ~

 * _Get Started with Rultlang_
   * https://rustup.rs/
 * _Get Started with RocketAPI_
   * https://rocket.rs/


### Quick Start
```shell

# install rustlang
# ~~~~~~~~~~~~~~~~
# ~ WARNING:     ~
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# ~ You're piping a script to sh                                             ~
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
curl https://sh.rustup.rs -sSf | sh
# >> OPPORTUNITY: <<
# >> choose the nightly during install <<
# >> you'll have the opportunity to switch w. the RocketAPI install <<

# source cargo ~ the rustlang build toolkit
source $HOME/.cargo/env

# clone this repo
git clone https://github.com/virtualadrian/rocketapi-starter.git

# jump on in
cd rocketapi-starter/

# run the 'app' and prepare to wait!
cargo run

# curl index
curl http://localhost:8000

```


### Switch to Nightly

```shell

# change default
rustup default nightly

# oveerride to nightly fif needed
rustup override set nightly

#update Rustlang
rustup update

# update Cargo
cargo update

```


### Environments and Rocket.toml

**Explore Rocket.toml for environment specific settings.**
> To Launch Staging:
>```shell
> ROCKET_ENV=staging ./target/debug/rocketapi-starter
>
> curl http://localhost:9000
> ```


#### Next Steps:
> Look into data and ORM  
> https://github.com/diesel-rs/diesel

#### Note:
> This is my first walkthrough, Google would be faster unless a question/issue is
> in response to an error on my part, or an improvement.
>
> All constructive input is welcome!
>

#### More From The Authors :
> https://rocket.rs/


#### Current Versions:
> * ~ Rocket v0.3.2 (Aug 15, 2017)
> * ~ Rustlang 1.22.0-nightly (2017-09-14)
