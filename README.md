# Vera_Rig


![Vera_rig](https://github.com/user-attachments/assets/0e9efce3-09d7-47b0-8573-47ff9385a9a2)


A Rust-based AI agent implementation using [rig](https://github.com/0xPlaygrounds/rig) for AI functionality, powering an autonomous social media presence on X (formerly Twitter).

Follow our AI agent: [@Vera_Rig_RIG](https://x.com/Vera_Rig_RIG)

## Overview

Vera_Rig is a cutting-edge AI social media agent designed to autonomously engage with users, craft posts, and interact on platforms like X. Built in Rust for performance and reliability, Vera_Rig leverages the rig framework to provide AI-driven functionality, ensuring natural conversation flow and a consistent, dynamic personality.

## Key Features

### Character-Based Design
- Consistently expresses character traits through predefined attributes.
- Configure unique voice and content preferences for diverse engagement.
- Tailored replies based on the character's profile, creating an immersive experience.

### Autonomous Interaction
- Creates engaging, contextually relevant posts with original content.
- Reacts thoughtfully to mentions, interactions, and discussions.
- Filters responses intelligently to ensure meaningful interactions.
- Maintains fluid and authentic communication with users.

### Advanced Memory System
- Stores interaction history for personalized engagement over time.
- Provides responses that acknowledge past conversations and interactions.
- Monitors interactions with users, building relationships and adjusting behavior.

### Platform Integration
- Built-in rate limiting and scheduling
- Random delays for natural posting patterns
- Comprehensive Twitter API v2 integration

### Modular Architecture
- Clear division between core functionality and platform integrations.
- Easily extend and modify character traits for diverse personalities.
- Integrate additional services and functionalities.
- Optimized for performance while handling large data sets.

## Prerequisites

- Rust (latest stable version)
- API Keys:
  - Anthropic Claude API access
  - Twitter API v2 credentials (OAuth 1.0a)

## Installation

1. Clone the repository:
git clone https://github.com/chakaboommm/Vera_Rig
cd Vera_Rig

2. Create a `.env` file with required credentials:
ANTHROPIC_API_KEY=your_api_key
TWITTER_CONSUMER_KEY=your_key
TWITTER_CONSUMER_SECRET=your_secret
TWITTER_ACCESS_TOKEN=your_token
TWITTER_ACCESS_TOKEN_SECRET=your_token_secret
CHARACTER_NAME=your_character_name

3. Configure your character:
   - Create a new directory: `characters/{CHARACTER_NAME}/`
   - Add character definition in `character.json`

## Character Configuration

Characters are defined using a structured JSON format:

{
  "instructions": {
    "base": "Base character instructions",
    "suffix": "Additional instructions"
  },
  "adjectives": ["trait1", "trait2"],
  "bio": {
    "headline": "Character headline",
    "key_traits": ["trait1", "trait2"]
  },
  "lore": ["background1", "background2"],
  "styles": ["style1", "style2"],
  "topics": ["topic1", "topic2"],
  "post_style_examples": ["example1", "example2"]
}

## Usage

Run the agent:
cargo run

## Project Structure

Vera_Rig/
├── src/
│   ├── core/           # Core agent functionality
│   ├── characteristics/ # Character trait implementations
│   ├── providers/      # External service integrations
│   └── memory/         # Persistence layer
├── characters/         # Character definitions
└── tests/             # Test suite

## Dependencies

- [rig](https://github.com/0xPlaygrounds/rig) - AI agent framework
- `twitter-v2` - Twitter API client
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `anyhow` - Error handling


## Acknowledgments

- [rig](https://github.com/0xPlaygrounds/rig) team for the AI agent framework
- Contributors and maintainers

## Support

For questions and support, please open an issue in the GitHub repository.
