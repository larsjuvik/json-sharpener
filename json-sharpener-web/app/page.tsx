"use client";
import { ChangeEvent, useEffect, useState } from "react";
import Editor from "react-simple-code-editor";
import Prism, { highlight } from "prismjs";
import "prismjs/themes/prism-okaidia.css";
import "prismjs/components/prism-json";
import "prismjs/components/prism-csharp";

export default function Home() {
  const [inputText, setInputText] = useState(`{
    "name": "Tester",
    "age": 25,
    "isHappy": true,
    "dinners": ["pizza", "taco"]
}`);
  const [outputText, setOutputText] = useState("");
  const [errorText, setErrorText] = useState<string | undefined>();

  const [convertJsonToCSharp, setConvertJsonToCSharp] = useState<
    undefined | ((json: string) => string)
  >();
  const [convertJsonToCSharpError, setConvertJsonToCSharpError] = useState<
    undefined | ((json: string) => string)
  >();

  // Load WASM library
  useEffect(() => {
    const loadWasm = async (): Promise<
      | undefined
      | {
          convert: (json: string) => string;
          convertError: (json: string) => string;
        }
    > => {
      try {
        const libraryModule = await import("@/public/wasm/json_sharpener_wasm");
        const libUrl = "/wasm/json_sharpener_wasm_bg.wasm";
        await libraryModule.default(libUrl);
        return {
          convert: libraryModule.convert_json_to_csharp,
          convertError: libraryModule.convert_json_to_csharp_error,
        };
      } catch (error) {
        console.error("Failed to load WASM module", error);
      }
    };

    loadWasm().then((r) => {
      if (r !== undefined) {
        setConvertJsonToCSharp(() => r.convert);
        setConvertJsonToCSharpError(() => r.convertError);
        setOutputText(r.convert(inputText));
      }
    });
  }, []);

  useEffect(() => {
    if (convertJsonToCSharp === undefined) return;
    if (convertJsonToCSharpError === undefined) return;

    const csharpText = convertJsonToCSharp(inputText);
    if (!csharpText && inputText) {
      // Conversion not available
      setOutputText("Enter valid JSON");
    } else {
      setOutputText(csharpText);
    }

    setErrorText(convertJsonToCSharpError(inputText) ?? "");
  }, [inputText]);

  return (
    <div className="ms-auto me-auto flex flex-col items-center justify-items-between h-full p-8 pb-0 gap-16 sm:p-10 sm:pb-0 max-w-6xl">
      <main className="w-full flex flex-col gap-8 row-start-2 items-stretch">
        <div>
          <h1 className="text-lg">JSON Sharpener</h1>
          <h2 className=" text-sm">
            Powered by <span className="text-orange-700">Rust</span> and{" "}
            <span className="text-blue-700">WebAssembly</span>
          </h2>
        </div>
        <div className="flex flex-1 flex-col sm:flex-row items-stretch justify-between gap-5 md:gap-10">
          <Editor
            className="border-2 border-slate-700 rounded flex-1 bg-slate-800"
            value={inputText}
            onValueChange={(code) => setInputText(code)}
            highlight={(code) => highlight(code, Prism.languages.json, "json")}
            padding={10}
            style={{
              fontFamily: "monospace",
              fontSize: 12,
            }}
          />
          <Editor
            className="border-2 border-slate-700 rounded flex-1 bg-slate-800"
            value={outputText}
            onValueChange={(code) => setOutputText(code)}
            highlight={(code) =>
              Prism.highlight(code, Prism.languages.csharp, "csharp")
            }
            padding={10}
            style={{
              fontFamily: "monospace",
              fontSize: 12,
            }}
          />
        </div>
        <p className="text-amber-700">{errorText}</p>
      </main>
      <footer className="w-full text-white py-2 text-center mt-auto">
        <p className="text-xs">
          © 2024 JSON Sharpener. All rights reserved.{" "}
          <span
            className="font-bold hover:cursor-pointer"
            onClick={() =>
              alert(
                'This tool is provided "as-is" without any warranties or guarantees. We are not liable for any errors or damages. Errors can occur, always double check the results. Use at your own risk.'
              )
            }>
            See disclaimer
          </span>
        </p>
      </footer>
    </div>
  );
}
