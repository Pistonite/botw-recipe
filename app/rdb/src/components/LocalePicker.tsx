import { useEffect, useState } from "react";
import { Menu, MenuButton, MenuItemRadio, MenuList, MenuPopover, MenuTrigger } from "@fluentui/react-components";
import { Globe20Regular} from "@fluentui/react-icons";

import { loadLocalePreference, SupportedLocales, switchLanguage } from "i18n/locales.ts";
import { useHost } from "host/HostProvider.ts";

/** Language Picker */
export const LocalePicker: React.FC = () => {
    const [locale, setLocale] = useState(loadLocalePreference);

    const host = useHost();

    useEffect(() => {
        switchLanguage(locale, host);
    }, [locale]);

    return (
        <Menu checkedValues={{locale: [locale]}} onCheckedValueChange={(_, {checkedItems}) => {
            setLocale(checkedItems[0]);
        }}>
            <MenuTrigger disableButtonEnhancement>
                <MenuButton appearance="subtle" icon={<Globe20Regular/>} />
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {
                        Object.entries(SupportedLocales).map(([key, value]) => (
                            <MenuItemRadio key={key} name="locale" value={key}>{value}</MenuItemRadio>
                        ))
                    }
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
