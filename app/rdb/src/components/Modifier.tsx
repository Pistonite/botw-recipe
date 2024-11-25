/**
 * Modifier related components
 */
import { useTranslation } from "react-i18next";
import { memo, useState } from "react";
import {
    Label,
    type LabelProps,
    makeStyles,
    Tooltip,
} from "@fluentui/react-components";

import { WeaponModifiers, type WeaponModifier } from "data/WeaponModifier.ts";
import type { WeaponModifierSet } from "host/types.ts";

const useStyles = makeStyles({
    iconContainer: {
        backgroundImage: 'url("/modifiers/bg.png")',
        minWidth: "26px",
        width: "26px",
        minHeight: "26px",
        height: "26px",
        padding: "1px",
    },
    icon: {
        minWidth: "24px",
        width: "24px",
        minHeight: "24px",
        height: "24px",
    },
    dataContainer: {
        display: "flex",
    },
});

export type ModifierProps = {
    /** which modifier to display */
    modifier: WeaponModifier;
};

/** A localized modifier label */
export const ModifierLabel: React.FC<ModifierProps & LabelProps> = ({
    modifier,
    ...rest
}) => {
    const { t } = useTranslation();
    return <Label {...rest}>{t(`modifier.${modifier}`)}</Label>;
};

export type ModifierDataProps = {
    modifiers: WeaponModifierSet;
    value: number;
};

export const ModifierData: React.FC<ModifierDataProps> = memo(
    ({ modifiers, value }) => {
        const styles = useStyles();
        return (
            <div className={styles.dataContainer}>
                {WeaponModifiers.map((modifier) => {
                    if (!(modifiers & modifier)) {
                        return null;
                    }
                    return (
                        <ModifierWithValueTooltip
                            key={modifier}
                            modifier={modifier}
                            value={value}
                        />
                    );
                })}
            </div>
        );
    },
);

export const ModifierWithValueTooltip: React.FC<
    ModifierProps & { value: number }
> = ({ modifier, value }) => {
    const [ref, setRef] = useState<HTMLSpanElement | null>(null);
    const { t } = useTranslation();
    const values = {
        value,
        valueThousandth: (value / 1000).toFixed(3),
        valueMax10: Math.min(value, 10),
    };
    return (
        <Tooltip
            positioning={{ target: ref }}
            appearance="inverted"
            content={t(`modifier.${modifier}.value`, values)}
            relationship="label"
        >
            <span ref={setRef}>
                <Modifier modifier={modifier} />
            </span>
        </Tooltip>
    );
};

/** Component that displays a weapon modifier block */
export const Modifier: React.FC<ModifierProps> = memo(({ modifier }) => {
    const styles = useStyles();

    return (
        <div className={styles.iconContainer} aria-hidden>
            <img className={styles.icon} src={`/modifiers/${modifier}.png`} />
        </div>
    );
});
