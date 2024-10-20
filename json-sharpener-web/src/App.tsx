"use client";
import { useEffect, useState } from "react";
import Editor from "react-simple-code-editor";
import Prism, { highlight } from "prismjs";
import "prismjs/themes/prism-okaidia.css";
import "prismjs/components/prism-json";
import "prismjs/components/prism-csharp";
import { useJsonSharpener } from "./hooks/useJsonSharpener";

export default function Home() {
  const functions = useJsonSharpener();
  const [inputText, setInputText] = useState(`{
    "name": "Tester",
    "age": 25,
    "isHappy": true,
    "dinners": ["pizza", "taco"]
}`);
  const [outputText, setOutputText] = useState("");
  const [errorText, setErrorText] = useState<string | undefined>();

  // When input text changes
  useEffect(() => {
    if (functions === undefined) return;
    setOutputText(functions.convertJsonToCSharp(inputText, "TestClass"));
    setErrorText(functions.convertJsonToCSharpError(inputText));
  }, [inputText]);

  // When library is loaded
  useEffect(() => {
    if (functions === undefined) return;
    setOutputText(functions.convertJsonToCSharp(inputText, "TestClass"));
    setErrorText(functions.convertJsonToCSharpError(inputText));
  }, [functions]);

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
        <div className="flex flex-1 flex-col md:flex-row items-stretch justify-between gap-5 md:gap-10">
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
            onValueChange={(_) => {}}
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
