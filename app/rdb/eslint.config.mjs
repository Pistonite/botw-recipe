// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

function overrideRule(configs, overrides) {
    if (Array.isArray(configs)) {
        for (const config of configs) {
            overrideRule(config, overrides);
        }
    } else {
        if (configs.rules) {
            for (const rule in overrides) {
                if (configs.rules[rule]) {
                    configs.rules[rule] = overrides[rule];
                }
            }
        }
    }
    return configs;
}

const config = tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.strict,
);

const overrides = {
    // typescript has coverage already
    "@typescript-eslint/no-unused-vars": "off",
};


export default overrideRule(config, overrides);
