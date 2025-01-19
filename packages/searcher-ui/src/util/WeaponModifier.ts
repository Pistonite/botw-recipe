import type { WeaponModifierSet } from "botw-recipe-searcher-tauri";

/**
 * Weapon modifier bitset implementation
 */
export const WeaponModifier = {
    None: 0,
    AddPower: 1,
    AddLife: 2,
    Critical: 4,
    AddThrow: 8,
    SpreadFire: 0x10,
    Zoom: 0x20,
    RapidFire: 0x40,
    SurfMaster: 0x80,
    AddGuard: 0x100,
} as const;

export const WeaponModifiers = [
    WeaponModifier.AddPower,
    WeaponModifier.AddLife,
    WeaponModifier.Critical,
    WeaponModifier.AddThrow,
    WeaponModifier.SpreadFire,
    WeaponModifier.Zoom,
    WeaponModifier.RapidFire,
    WeaponModifier.SurfMaster,
    WeaponModifier.AddGuard,
] as const;

export type WeaponModifier =
    (typeof WeaponModifier)[keyof typeof WeaponModifier];

export const hasMultishotAndDoesNotExcludeQuickShot = (
    include: WeaponModifierSet,
    exclude: WeaponModifierSet,
) => {
    return (
        (include & WeaponModifier.SpreadFire) !== 0 &&
        (exclude & WeaponModifier.RapidFire) === 0
    );
};
