import {
    Menu,
    MenuButton,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
} from "@fluentui/react-components";
import { Globe20Regular } from "@fluentui/react-icons";
import { useLocale } from "@pistonite/pure-react";
import { getLocalizedLanguageName } from "@pistonite/pure/pref";

import { SupportedLocales, switchLanguage } from "i18n/locales.ts";
import { useHost } from "host/useHost.ts";

/** Language Picker */
export const LocalePicker: React.FC = () => {
    const locale = useLocale();
    const host = useHost();

    return (
        <Menu
            checkedValues={{ locale: [locale] }}
            onCheckedValueChange={(_, { checkedItems }) => {
                switchLanguage(checkedItems[0], host);
            }}
        >
            <MenuTrigger disableButtonEnhancement>
                <MenuButton appearance="subtle" icon={<Globe20Regular />} />
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {SupportedLocales.map((key) => (
                        <MenuItemRadio key={key} name="locale" value={key}>
                            {getLocalizedLanguageName(key)}
                        </MenuItemRadio>
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
