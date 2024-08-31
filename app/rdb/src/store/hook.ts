import { 
    useDispatch as useReduxDispatch,
    useSelector as useReduxSelector
} from "react-redux";
import type { Dispatch, State } from "./store.ts";

export const useDispatch = useReduxDispatch.withTypes<Dispatch>();
export const useSelector = useReduxSelector.withTypes<State>();
