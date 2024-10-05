"use client";
import { ChangeEvent, useState } from "react";
import Editor from "react-simple-code-editor";
import Prism, { highlight } from "prismjs";
import "prismjs/themes/prism.css";
import "prismjs/components/prism-json";
import "prismjs/components/prism-csharp";
import init, { convert_json_to_csharp } from "@/lib/wasm/json_sharpener_wasm";

export default function Home() {
  const [libraryLoaded, setLibraryLoaded] = useState(false);
  const [inputText, setInputText] = useState("");
  const [outputText, setOutputText] = useState("");

  async function loadLibrary() {
    await init();
    setLibraryLoaded(true);
  }
  loadLibrary();

  function inputTextChanged(event: ChangeEvent<HTMLTextAreaElement>): void {
    if (!libraryLoaded) return;
    const newOutput = convert_json_to_csharp(event.target.value);
    setOutputText(newOutput);
  }

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <h1>JSON Sharpener</h1>
        <h2>WASM Library Loaded: {libraryLoaded}</h2>
        <div className="flex flex-row gap-10">
          <Editor
            value={inputText}
            onValueChange={(code) => setInputText(code)}
            highlight={(code) =>
              highlight(code, Prism.languages.csharp, "csharp")
            }
            padding={10}
            style={{
              fontFamily: "monospace",
              fontSize: 12,
            }}
          />
          <Editor
            value={outputText}
            onValueChange={(code) => setOutputText(code)}
            highlight={(code) =>
              Prism.highlight(code, Prism.languages.json, "json")
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
