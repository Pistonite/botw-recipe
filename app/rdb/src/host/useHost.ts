import { createContext, useContext } from "react";
import type { Host } from "./Host.ts";

export const HostContext = createContext<Host>(null as unknown as Host);

export const useHost = () => useContext(HostContext);
