# Environment Variables for UniswapX Artemis

## Required Environment Variables

### Network Configuration
```bash
# Story node WebSocket endpoint for real-time data (optional)
WSS_ENDPOINT=wss://arbitrum-one.publicnode.com

# Story node HTTP endpoint for transactions  
HTTP_ENDPOINT=https://aeneid.storyrpc.io

# Chain ID (1315 Story testnet, 1514 Story)
CHAIN_ID=1315
```

### Private Key Configuration (choose one method)
```bash
# Option 1: Direct private key (not recommended for production)
PRIVATE_KEY=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

# Option 2: Path to file containing address->private key mapping
PRIVATE_KEY_FILE=/path/to/keyfile.json

# Option 3: AWS Secrets Manager ARN for private key
AWS_SECRET_ARN=arn:aws:secretsmanager:us-east-1:123456789012:secret:uniswapx-keys
```

### Executor Configuration
```bash
# Executor contract address for order execution
EXECUTOR_ADDRESS=0x1234567890123456789012345678901234567890

# Order type to process (Limit, DutchV2, DutchV3, Priority)
ORDER_TYPE=Limit
```

## Optional Environment Variables

### Strategy Configuration
```bash
# Percentage of profit to bid in gas (default: 90%)
BID_PERCENTAGE=90

# Fallback bid scale factor (default: 100 = 1% of profit)
FALLBACK_BID_SCALE_FACTOR=100

# Minimum block percentage buffer for priority orders (default: 120 = 120% of block time)
MIN_BLOCK_PERCENTAGE_BUFFER=120
```

### Reactor Addresses (strategy-specific)
```bash
# General reactor address (used as fallback)
REACTOR_ADDRESS=0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87

# Strategy-specific reactor addresses (override general one)
REACTOR_ADDRESS_LIMIT=0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87
REACTOR_ADDRESS_DUTCHV3=0xB274d5F4b833b61B340b654d600A864fB604a87c
REACTOR_ADDRESS_UNISWAP=0x00000011F84B9aa48e5f8aA8B9897600006289Be
REACTOR_ADDRESS_PRIORITY=0x000000001Ec5656dcdB24D90DFa42742738De729
```

### API Endpoints
```bash
# Routing API for getting swap routes
ROUTING_API=https://router-dev.mimboku.com/quote

# Mimboku API for fetching orders
MIMBOKU_API_URL=https://j6nfzv9j4e.execute-api.us-east-1.amazonaws.com/prod/limit

# Mimboku API key (optional)
MIMBOKU_API_KEY=your-uniswapx-api-key-here
```

### AWS Configuration
```bash
# Enable CloudWatch metrics (true/false)
CLOUDWATCH_METRICS=false

# AWS region (if using AWS features)
AWS_REGION=us-east-1
```

## Example Configurations

### Story testnet configuration for
```bash
HTTP_ENDPOINT=https://aeneid.storyrpc.io
CHAIN_ID=1315
ORDER_TYPE=Limit
BID_PERCENTAGE=1
CLOUDWATCH_METRICS=false
PRIVATE_KEY=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
EXECUTOR_ADDRESS=0x25327881066a09F12B066ceA76ADaa1ca61E6c82
REACTOR_ADDRESS=0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87
ROUTING_API=https://router-dev.mimboku.com/quote
MIMBOKU_API_URL=https://j6nfzv9j4e.execute-api.us-east-1.amazonaws.com/prod/limit
MIMBOKU_API_KEY=your-uniswapx-api-key-here
```

## Usage

1. Copy these variables to a `.env` file in your project root
2. Update the values with your actual configuration
3. Run the application:
   ```bash
   # Load environment variables and run
   source .env && cargo run
   
   # Or export all variables
   export $(cat .env | xargs) && cargo run
   ```

## Security Notes

- **Never commit `.env` files** with real private keys to version control
- Use AWS Secrets Manager for production private key management
- Consider using different executor addresses for different environments
- Regularly rotate private keys and API keys 