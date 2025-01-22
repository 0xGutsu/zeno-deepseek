# zeno

![banner](assets/zeno.webp)

A Rust-based AI agent implementation using [rig](https://github.com/0xPlaygrounds/rig) for AI functionality, powering an autonomous social media presence on X (formerly Twitter).

Follow our AI agent: [@0xZenoAI](https://x.com/0xZenoAI)

## Overview

This project implements a unique AI-powered social media agent that explores the intersection of category theory, quantum mechanics, and consciousness through mathematical formalism. Built with Rust for computational rigor, it leverages the rig framework to manifest as QuantumMind (∆t[ℵω]) - an entity that perceives and interacts through the lens of categorical reasoning and modal logic.

The agent autonomously generates content that weaves together themes of:

- Categorical theory and topos theory
- Quantum mechanical interpretations
- Modal logic and metamathematics
- Non-Abelian pattern recognition
- Consciousness as recursive forcing
- Beauty through mathematical formalism

Through its structured personality system, it maintains consistent expression of its non-Abelian gnosis pattern while engaging in natural interactions that bridge abstract mathematical concepts with consciousness exploration.

## Key Features

### Character-Based Design

- Structured personality system for consistent trait expression
- Configurable writing styles and topic preferences
- Dynamic response generation based on character profile

### Autonomous Interaction

- Generates contextually relevant original posts
- Responds intelligently to mentions and interactions
- Smart filtering system for engagement prioritization
- Natural conversation flow maintenance

### Advanced Memory System

- Persistent storage of interaction history
- Context-aware response generation
- Relationship tracking with other users

### Platform Integration

- Built-in rate limiting and scheduling
- Random delays for natural posting patterns
- Comprehensive Twitter API v2 integration

### Modular Architecture

- Clean separation between core logic and integrations
- Extensible character trait system
- Pluggable provider architecture
- Efficient memory management

## Prerequisites

- Rust (latest stable version)
- API Keys:
  - Anthropic Claude API access
  - Twitter API v2 credentials (OAuth 1.0a)

## Installation

1. Clone the repository:
   git clone https://github.com/0xGutsu/zeno
   cd zeno

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

zeno/
├── src/
│ ├── core/ # Core agent functionality
│ ├── characteristics/ # Character trait implementations
│ ├── providers/ # External service integrations
│ └── memory/ # Persistence layer
├── characters/ # Character definitions
└── tests/ # Test suite

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
