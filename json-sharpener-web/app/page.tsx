"use client";
import init, { convert_json_to_csharp } from "@/lib/wasm/json_sharpener_wasm";
import { ChangeEvent, useState } from "react";

export default function Home() {
  const [libraryLoaded, setLibraryLoaded] = useState(false);
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
          <textarea
            className="resize rounded border border-purple-600 bg-slate-700 text-blue-500"
            onChange={inputTextChanged}></textarea>
          <textarea
            className="resize rounded border border-purple-600 bg-slate-700 text-blue-500"
            value={outputText}
            readOnly></textarea>
        </div>
      </main>
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center"></footer>
    </div>
  );
}
