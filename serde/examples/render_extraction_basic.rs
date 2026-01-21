//! Basic manual extraction example
//!
//! This example demonstrates the simplest approach: manually extracting
//! renderable primitives using a wrapper type.
//!
//! Run with: cargo run --example render_extraction_basic

use std::collections::HashMap;

// ============================================================================
// Rendering Primitives
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Red,
    Blue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Circle { position: Vec2, radius: f32, color: Color },
    Sprite { position: Vec2, sprite_id: String, team_color: Color },
}

// ============================================================================
// Renderable Trait
// ============================================================================

pub trait Renderable {
    fn to_primitive(&self) -> Primitive;
}

// ============================================================================
// Game State
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Red,
    Blue,
}

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Renderable for Ball {
    fn to_primitive(&self) -> Primitive {
        Primitive::Circle {
            position: self.position.clone(),
            radius: self.radius,
            color: Color::White,
        }
    }
}

pub struct Player {
    pub position: Vec2,
    pub team: Team,
    pub is_selected: bool,
}

impl Renderable for Player {
    fn to_primitive(&self) -> Primitive {
        let team_color = match self.team {
            Team::Red => Color::Red,
            Team::Blue => Color::Blue,
        };
        Primitive::Sprite {
            position: self.position.clone(),
            sprite_id: "player".to_string(),
            team_color,
        }
    }
}

pub struct SoccerGame {
    pub ball: Ball,
    pub players: Vec<Player>,
    pub score: [u32; 2],
}

// ============================================================================
// Manual Extraction
// ============================================================================

impl SoccerGame {
    /// Manually extract all renderable primitives from the game state
    pub fn extract_renderables(&self) -> HashMap<String, Primitive> {
        let mut result = HashMap::new();

        // Extract ball
        result.insert("ball".to_string(), self.ball.to_primitive());

        // Extract players with array indices
        for (i, player) in self.players.iter().enumerate() {
            result.insert(format!("players[{}]", i), player.to_primitive());
        }

        result
    }
}

// ============================================================================
// Example
// ============================================================================

fn main() {
    println!("=== Basic Manual Extraction Example ===\n");

    // Create a game state
    let game = SoccerGame {
        ball: Ball {
            position: Vec2 { x: 50.0, y: 30.0 },
            velocity: Vec2 { x: 1.0, y: 0.5 },
            radius: 2.0,
        },
        players: vec![
            Player {
                position: Vec2 { x: 20.0, y: 30.0 },
                team: Team::Red,
                is_selected: false,
            },
            Player {
                position: Vec2 { x: 80.0, y: 30.0 },
                team: Team::Blue,
                is_selected: true,
            },
        ],
        score: [0, 0],
    };

    // Extract renderables
    let primitives = game.extract_renderables();

    // Display results
    println!("Extracted {} rendering primitives:\n", primitives.len());
    for (path, primitive) in &primitives {
        match primitive {
            Primitive::Circle { position, radius, color } => {
                println!("  {} -> Circle at ({}, {}) radius {:.1} color {:?}",
                         path, position.x, position.y, radius, color);
            }
            Primitive::Sprite { position, sprite_id, team_color } => {
                println!("  {} -> Sprite '{}' at ({}, {}) color {:?}",
                         path, sprite_id, position.x, position.y, team_color);
            }
        }
    }

    println!("\n✓ Manual extraction complete!");
    println!("\nPros:");
    println!("  - Simple and explicit");
    println!("  - Full control over extraction logic");
    println!("  - Zero dependencies");
    println!("\nCons:");
    println!("  - Manual implementation for each type");
    println!("  - Easy to forget to extract new fields");
    println!("  - Repetitive code");
}
