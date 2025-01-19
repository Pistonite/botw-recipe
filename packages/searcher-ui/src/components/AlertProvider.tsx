/**
 * Alert dialog system
 */

import {
    createContext,
    type PropsWithChildren,
    useCallback,
    useContext,
    useState,
} from "react";
import {
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
} from "@fluentui/react-components";
import { useTranslation, getErrorTitle, getErrorMessage, getErrorButtonText } from "botw-recipe-searcher-localization";
import { HostError } from "botw-recipe-searcher-tauri";

/** Payload for triggering an alert */
export type AlertPayload = {
    /** Title of the alert */
    title: string;
    /** Body message of the alert */
    message: string;
    /** Actions to display on the alert */
    actions: string[];
};

export const getErrorAlertPayload = (error: HostError): AlertPayload => {
    return {
        title: getErrorTitle(),
        message: getErrorMessage(error),
        actions: [getErrorButtonText()],
    };
};

export type AlertFn = (payload: AlertPayload) => Promise<number>;

const AlertContext = createContext<AlertFn>(async () => 0);

export const useAlert = () => useContext(AlertContext);
export const useConfirm = (localizedMessage: string) => {
    const t = useTranslation();
    const alert = useAlert();
    return useCallback(async () => {
        const confirmAction = await alert({
            title: t("confirm.title"),
            message: localizedMessage,
            actions: [t("confirm.button.no"), t("confirm.button.yes")],
        });
        return confirmAction === 1;
    }, [t, alert, localizedMessage]);
};

export const AlertProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const [open, setOpen] = useState(false);
    const [title, setTitle] = useState("");
    const [message, setMessage] = useState("");
    const [actions, setActions] = useState<string[]>([]);
    const [resolve, setResolve] = useState<((value: number) => void) | null>(
        null,
    );

    const alertFn: AlertFn = useCallback(
        ({ title, message, actions }: AlertPayload) => {
            return new Promise<number>((resolve) => {
                setOpen(true);
                setTitle(title);
                setMessage(message);
                setActions(actions);
                setResolve(() => resolve);
            });
        },
        [],
    );

    return (
        <AlertContext.Provider value={alertFn}>
            {children}

            <Dialog
                modalType="alert"
                open={open}
                onOpenChange={(_, data) => {
                    if (!data.open) {
                        setOpen(false);
                    }
                }}
            >
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{title}</DialogTitle>
                        <DialogContent>{message}</DialogContent>
                        <DialogActions>
                            {actions.map((action, index) => (
                                <DialogTrigger
                                    key={index}
                                    disableButtonEnhancement
                                >
                                    <Button
                                        appearance={
                                            index === actions.length - 1
                                                ? "primary"
                                                : "secondary"
                                        }
                                        onClick={() => {
                                            // let resolve happen after UI updates
                                            setTimeout(() => {
                                                resolve?.(index);
                                            }, 0);
                                        }}
                                    >
                                        {action}
                                    </Button>
                                </DialogTrigger>
                            ))}
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </AlertContext.Provider>
    );
};
