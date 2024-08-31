/**
 * Alert dialog system
 */

import { createContext, type PropsWithChildren, useCallback, useContext, useState } from "react";
import { Button, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle, DialogTrigger } from "@fluentui/react-components";

/** Payload for triggering an alert */
export type AlertPayload = {
    /** Title of the alert */
    title: string,
    /** Body message of the alert */
    message: string,
    /** Actions to display on the alert */
    actions: string[],
}

export type AlertFn = (payload: AlertPayload) => Promise<number>;

const AlertContext = createContext<AlertFn>(async () => 0);

export const useAlert = () => useContext(AlertContext);

export const AlertProvider: React.FC<PropsWithChildren> = ({ children }) => {

    const [title, setTitle] = useState("");
    const [message, setMessage] = useState("");
    const [actions, setActions] = useState<string[]>([]);
    const [resolve, setResolve] = useState<((value: number) => void) | null>(null);

    const alertFn: AlertFn = useCallback(({title, message, actions}: AlertPayload) => {
        return new Promise<number>((resolve) => {
            setTitle(title);
            setMessage(message);
            setActions(actions);
            setResolve(() => resolve);
        });
    }, []);

    return (
    <AlertContext.Provider value={alertFn}>
            {children}

            <Dialog modalType="alert" open={!!title} onOpenChange={(_, data) => {
                if (!data.open) {
                    setTitle("");
                }
            }}>
            <DialogSurface>
            <DialogBody>
                        <DialogTitle>{title}</DialogTitle>
                        <DialogContent>{message}</DialogContent>
                        <DialogActions>
                            {actions.map((action, index) => (
                                <DialogTrigger key={index} disableButtonEnhancement>
                                    <Button 
                                        appearance={index === actions.length - 1 ? "primary" : "secondary"}
                                        onClick={() => {
                                            setTitle("");
                                            resolve?.(index);
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
