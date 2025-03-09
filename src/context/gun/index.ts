import { createContext } from "preact";
import { IGunContext } from "./types";

export const GunContext = createContext<IGunContext>(null);
