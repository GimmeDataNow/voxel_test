use bevy::prelude::*;

/// # Usage:
/// This is the general Block data format which is used in the world. It is designed to minimise the data that is carried by each instance/entity
/// 
/// # Fields:
#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Block {
    block_type: BlockType,
    face_direction: Facing,
    power_lvl: u8,
}

/// # Usage:
/// This ```Facing``` enum dictates what texture to render based on the direction
/// # Format:
/// ```
/// pub enum Facing {
///     XPositive,
///     XNegative,
///     YPositive,
///     YNegative,
///     ZPositive,
///     ZNegative
/// }
/// ```
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Facing {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
    ZPositive,
    ZNegative
}
/// # Usage:
/// This ```Transparency``` enum exist to mimic the classical minecraft behavior of semi-transparent blocks
/// # Format:
/// ```
/// pub enum Transparency {
///     NonOpaque,
///     Opaque
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Transparency {
    NonOpaque,
    Opaque
}

/// # Usage:
/// This ```LightEmission``` enum exist to mimic the classical minecraft behavior of semi-transparent blocks that emmit light, such as glowstone.
/// 
/// It will later be used to make the light engine
/// # Format:
/// ```
/// pub enum LightEmission {
///     None,
///     Some(u8)
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LightEmission {
    None,
    Some(u8)
}
/// # Usage:
/// This ```RedstonePowerLvl``` enum exist to mimic the redstone behavior
/// # Format:
/// ```
/// pub enum RedstonePowerLvl {
///     None,
///     Some(u8)
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RedstonePowerLvl {
    None,
    Some(u8)
}

/// # Usage:
/// This enumerates all listed ```BlockTypes``` to minimise the data size
/// # Format:
/// Just a list of Enums
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    Air,
    Stone,
    Dirt,
    RedstoneBlock
}

/// # Usage:
/// Merges all the data that can vary from block to block into one struct
/// # Format:
/// ```
/// pub struct BlockResource {
///     pub block_type: BlockType,
///     pub transparency: Transparency,
///     pub light_emission: LightEmission,
///     pub redstone_power_lvl: RedstonePowerLvl,
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BlockResource {
    pub block_type: BlockType,
    pub transparency: Transparency,
    pub light_emission: LightEmission,
    pub redstone_power_lvl: RedstonePowerLvl,
}

/// # Usage:
/// Merges the possible states into one ```LOOKUPTABLE``` for ease of use and repeated access
/// # Format:
/// ```
/// BlockResource{ block_type: BlockType::Air, transparency: Transparency::Opaque, light_emission: LightEmission::None, redstone_power_lvl: RedstonePowerLvl::None}
/// 
/// ```
/// # Warning:
/// the items in the ```LOOKUPTABLE ``` must be in the same order as the ```BlockType``` enum
pub const LOOKUPTABLE: [BlockResource; 4] = [
    BlockResource{ block_type: BlockType::Air,              transparency: Transparency::Opaque,    light_emission: LightEmission::None,     redstone_power_lvl: RedstonePowerLvl::None},
    BlockResource{ block_type: BlockType::Stone,            transparency: Transparency::NonOpaque, light_emission: LightEmission::None,     redstone_power_lvl: RedstonePowerLvl::None},
    BlockResource{ block_type: BlockType::Dirt,             transparency: Transparency::NonOpaque, light_emission: LightEmission::None,     redstone_power_lvl: RedstonePowerLvl::None},
    BlockResource{ block_type: BlockType::RedstoneBlock,    transparency: Transparency::NonOpaque, light_emission: LightEmission::Some(7),  redstone_power_lvl: RedstonePowerLvl::Some(15)}
    ];

impl Block {
    /// # Usage:
    /// just some boilerplate code. You can also just use a filled-out ```Block``` struct
    pub fn new(block_type: BlockType, face_direction: Facing ) -> Block { 
        Block { 
            block_type, 
            face_direction, 
            power_lvl: 0
        } 
    }

    /// # Usage:
    /// returns the properties of the ```BlockType``` 
    /// # Warning: 
    /// it assumes the ```BlockType``` and ```LOOKUPTABLE``` are in the same order
    pub fn get_base_properties(&self) -> BlockResource{ LOOKUPTABLE[self.block_type as usize] }
}



// UNIT TESTS //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let test_instance = Block::new(BlockType::Air, Facing::XPositive);
        let test_instance2 = Block {block_type: BlockType::Air, face_direction: Facing::XPositive, power_lvl: 0};
        assert_eq!(test_instance, test_instance2)
    }

    #[test]
    fn test_get_properties() {
        let test_instance = Block::new(BlockType::Air, Facing::XPositive);
        let test_type = test_instance.get_base_properties();
        assert_eq!(BlockType::Air, test_type.block_type)
    }
    #[test]
    fn test_get_light_lvl_none() {
        let test_instance = Block::new(BlockType::Air, Facing::XPositive);
        let test_type = test_instance.get_base_properties();
        assert_eq!(LightEmission::None, test_type.light_emission)
    }
    #[test]
    fn test_get_light_lvl_some() {
        let test_instance = Block::new(BlockType::RedstoneBlock, Facing::XPositive);
        let test_type = test_instance.get_base_properties();
        assert_eq!(LightEmission::Some(7), test_type.light_emission)
    }
    #[test]
    fn test_get_redstone_power_lvl_none() {
        let test_instance = Block::new(BlockType::Air, Facing::XPositive);
        let test_type = test_instance.get_base_properties();
        assert_eq!(RedstonePowerLvl::None, test_type.redstone_power_lvl)
    }
    #[test]
    fn test_get_redstone_power_lvl_some() {
        let test_instance = Block::new(BlockType::RedstoneBlock, Facing::XPositive);
        let test_type = test_instance.get_base_properties();
        assert_eq!(RedstonePowerLvl::Some(15), test_type.redstone_power_lvl)
    }
}