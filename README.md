# rocketapi-starter

#### Followed a Rusty book and hopped on RocketAPI ... ~ this is the aftermath ~


 * _Get Started with Rultlang_
   * https://rustup.rs/
 * _Get Started with RocketAPI_
   * https://rocket.rs/


#### Terminal Ahead
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

#### Note:
This is my first walkthrough, Google would be faster unless a question/issue is 
in response to an error on my part, or an improvement.

All constructive input is welcome!



#### Current Versions:
> * ~ Rocket v0.3.2 (Aug 15, 2017)
> * ~ Rustlang 1.22.0-nightly (2017-09-14)
