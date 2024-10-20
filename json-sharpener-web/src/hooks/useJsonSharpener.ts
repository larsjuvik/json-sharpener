import { useEffect, useState } from "react";
import init, {
  convert_json_to_csharp,
  convert_json_to_csharp_error,
} from "../assets/wasm/json_sharpener_wasm";
interface JsonSharpenerFunctions {
  convertJsonToCSharp: (json: string, className: string) => string;
  convertJsonToCSharpError: (json: string) => string;
}

export function useJsonSharpener(): JsonSharpenerFunctions | undefined {
  const [functions, setFunctions] = useState<
    JsonSharpenerFunctions | undefined
  >();

  // Load WASM library
  useEffect(() => {
    async function loadLibrary() {
      try {
        await init();

        const loadedFunctions: JsonSharpenerFunctions = {
          convertJsonToCSharp: convert_json_to_csharp,
          convertJsonToCSharpError: convert_json_to_csharp_error,
        };
        setFunctions(loadedFunctions);
      } catch (error) {
        console.error("Failed to load WASM module", error);
      }
    }
    loadLibrary();
  }, []);

  return functions;
}
