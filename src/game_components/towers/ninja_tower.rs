use crate::{
    asset_system::AssetManager,
    game_components::{towers::Tower, GoldPile, BLOCK_SIZE},
    game_views::monsters::MonsterView,
};

use rand::Rng;

use ggez::{
    audio::SoundSource,
    graphics::{self, DrawParam},
    mint::Point2,
    Context, GameResult,
};

pub struct NinjaTower {
    pub position: [f32; 2],
    pub attack_cooldown: f32,
    pub strong_attack_cooldown: f32,
}

impl NinjaTower {
    pub const ATTACK_RANGE: f32 = 100.0; // Pixels.
    pub const ATTACK_TIMER: f32 = 2.0; // Seconds.
    pub const STRONG_ATTACK_TIMER: f32 = 10.0; // Seconds.
    pub const DAMAGE: f32 = 10.0;
    pub const STRONG_ATTACK_DAMAGE: f32 = 1000.0;

    pub fn new(position: [f32; 2]) -> NinjaTower {
        NinjaTower {
            position,
            attack_cooldown: 2.0,
            strong_attack_cooldown: 5.0,
        }
    }

    pub fn get_center_pos_abs(&self) -> [f32; 2] {
        [
            self.position[0] * BLOCK_SIZE + BLOCK_SIZE / 2.0,
            self.position[1] * BLOCK_SIZE + BLOCK_SIZE / 2.0,
        ]
    }

    fn position_is_in_attack_range(&self, position_abs: [f32; 2]) -> bool {
        let tower_center_pos_abs = self.get_center_pos_abs();

        let dx = tower_center_pos_abs[0] - position_abs[0];
        let dy = tower_center_pos_abs[1] - position_abs[1];

        dx * dx + dy * dy < NinjaTower::ATTACK_RANGE * NinjaTower::ATTACK_RANGE
    }

    fn draw_attack(
        &mut self,
        ctx: &mut Context,
        from_abs: [f32; 2],
        to_abs: [f32; 2],
    ) -> GameResult {
        let line = graphics::Mesh::new_line(
            ctx,
            &[from_abs, to_abs],
            3.0,
            graphics::Color::new(0.0, 1.0, 1.0, 1.0),
        )?;

        let location = Point2 { x: 0.0, y: 0.0 };

        graphics::draw(ctx, &line, DrawParam::default().dest(location))?;

        Ok(())
    }
}

impl Tower for NinjaTower {
    fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = Point2 {
            x: self.position[0] * BLOCK_SIZE - 5.0,
            y: self.position[1] * BLOCK_SIZE - 35.0,
        };

        graphics::draw(
            ctx,
            &asset_manager.tower_assets.tower_ninja_sprite,
            DrawParam::default().dest(location),
        )?;

        Ok(())
    }

    fn draw_abilities(
        &mut self,
        ctx: &mut Context,
        monster_views: &Vec<Box<dyn MonsterView>>, //TODO: workaround to make separating monster component/view easier.
    ) -> GameResult {
        for monster_view in monster_views.iter() {
            let monster_center = monster_view.get_monster().get_center_pos_abs();

            if self.position_is_in_attack_range(monster_center) {
                self.draw_attack(ctx, self.get_center_pos_abs(), monster_center)?;
            }
        }
        Ok(())
    }

    fn update(
        &mut self,
        elapsed: f32,
        monster_views: &mut Vec<Box<dyn MonsterView>>, //TODO: workaround to make separating monster component/view easier.
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    ) {
        self.attack_cooldown -= elapsed;
        self.strong_attack_cooldown -= elapsed;

        if self.attack_cooldown < 0.0 {
            self.attack_cooldown = 0.0;
        }
        if self.strong_attack_cooldown < 0.0 {
            self.strong_attack_cooldown = 0.0;
        }

        if self.attack_cooldown == 0.0 {
            let mut damage_dealt = false;
            for monster_view in monster_views.iter_mut() {
                if self.position_is_in_attack_range(monster_view.get_monster().get_center_pos_abs())
                {
                    damage_dealt = true;
                    monster_view.get_monster_mut().recieve_damage(
                        NinjaTower::DAMAGE,
                        gold_piles,
                        asset_manager,
                    );
                }
            }
            if damage_dealt {
                asset_manager
                    .tower_assets
                    .tower_attack_sound
                    .play()
                    .unwrap();
                self.attack_cooldown = NinjaTower::ATTACK_TIMER;
            }
        }
        if self.strong_attack_cooldown == 0.0 {
            if monster_views.len() > 0 {
                let num = rand::thread_rng().gen_range(0..monster_views.len());
                //let mut rng = rand::thread_rng();
                //let choice = monsters.choose(&mut rng).unwrap();
                //monsters[rand::thread_rng().gen_range(0..monsters.len())]
                monster_views[num].get_monster_mut().recieve_damage(
                    NinjaTower::STRONG_ATTACK_DAMAGE,
                    gold_piles,
                    asset_manager,
                );
                asset_manager
                    .tower_assets
                    .ninja_tower_strong_attack_sound
                    .play()
                    .unwrap();
                self.strong_attack_cooldown = NinjaTower::STRONG_ATTACK_TIMER;
            }
        }
    }

    fn get_block_position(&self) -> [f32; 2] {
        self.position
    }
}
