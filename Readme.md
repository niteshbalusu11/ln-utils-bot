# Lightning Utilites Bot

- Built https://t.me/ln_utils_bot as an alternative to @LNPingBot but utilities bot also can also probe a node.
- LNPingBot has been unreliable for me so built this as an alernative.

## Self-hosting Steps
- Create a bot from @BotFather
- Grab the API Key
- You will need to install rust https://www.rust-lang.org/
```
# Add a .env file and update values
cp .env.example .env

# Clone the repo
git clone https://github.com/niteshbalusu11/ln-utils-bot.git

# Change directory
cd ln-utils-bot

# Build and run
cargo run --release
```

### For docker
- There is a Dockerfile and a sample docker-compose.yaml file in repo.