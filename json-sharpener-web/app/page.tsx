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

  const [convertJsonToCSharp, setConvertJsonToCSharp] = useState<
    undefined | ((json: string) => string)
  >();

  // Load WASM library
  useEffect(() => {
    const loadWasm = async (): Promise<
      undefined | ((json: string) => string)
    > => {
      try {
        const libraryModule = await import("@/public/wasm/json_sharpener_wasm");
        const libUrl = "/wasm/json_sharpener_wasm_bg.wasm";
        await libraryModule.default(libUrl);
        return libraryModule.convert_json_to_csharp;
      } catch (error) {
        console.error("Failed to load WASM module", error);
      }
    };

    loadWasm().then((r) => {
      if (r !== undefined) {
        setConvertJsonToCSharp(() => r);
        setOutputText(r(inputText));
      }
    });
  }, []);

  useEffect(() => {
    if (convertJsonToCSharp === undefined) return;

    const csharpText = convertJsonToCSharp(inputText);
    if (!csharpText && inputText) {
      // Conversion not available
      setOutputText("Enter valid JSON");
    } else {
      setOutputText(csharpText);
    }
  }, [inputText]);

  return (
    <div className="items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <div>
          <h1>JSON Sharpener</h1>
          <h2>Powered by Rust and WebAssembly</h2>
        </div>
        <div className="flex flex-1 flex-row justify-between gap-5">
          <Editor
            className="border border-purple-700 rounded w-max bg-slate-800"
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
            className="border border-purple-700 rounded w-max bg-slate-800"
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
      </main>
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center"></footer>
    </div>
  );
}
