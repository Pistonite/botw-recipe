import { createContext, useContext } from "react";
import { Host } from "./host";

export const HostContext = createContext<Host>(null as unknown as Host);

export const useHost = () => useContext(HostContext);
