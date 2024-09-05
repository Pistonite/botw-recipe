import {
    hasMultishotAndDoesNotExcludeQuickShot,
    WeaponModifier,
} from "./WeaponModifier";

const W = WeaponModifier;

describe("WeaponModifier", () => {
    test.each`
        include                      | exclude                 | expected
        ${0}                         | ${0}                    | ${false}
        ${W.SpreadFire}              | ${0}                    | ${true}
        ${W.SpreadFire}              | ${W.RapidFire}          | ${false}
        ${W.SpreadFire | W.AddPower} | ${0}                    | ${true}
        ${W.SpreadFire | W.AddPower} | ${W.RapidFire | W.Zoom} | ${false}
        ${W.AddPower}                | ${0}                    | ${false}
        ${W.RapidFire}               | ${0}                    | ${false}
    `(
        "hasMultishotAndDoesNotExcludeQuickShot",
        ({ include, exclude, expected }) => {
            expect(
                hasMultishotAndDoesNotExcludeQuickShot(include, exclude),
            ).toBe(expected);
        },
    );
});
