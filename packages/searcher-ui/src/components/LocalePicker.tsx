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
import { getLocalizedLanguageName, setLocale } from "@pistonite/pure/pref";
import { SupportedLocales } from "botw-recipe-searcher-localization";

/** Language Picker */
export const LocalePicker: React.FC = () => {
    const locale = useLocale();

    return (
        <Menu
            checkedValues={{ locale: [locale] }}
            onCheckedValueChange={(_, { checkedItems }) => {
                setLocale(checkedItems[0]);
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
