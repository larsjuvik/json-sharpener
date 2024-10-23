"use client";
import { useEffect, useState } from "react";
import Editor from "react-simple-code-editor";
import Prism, { highlight } from "prismjs";
import "prismjs/themes/prism-okaidia.css";
import "prismjs/components/prism-json";
import "prismjs/components/prism-csharp";
import { useJsonSharpener } from "./hooks/useJsonSharpener";
import { Button } from "./components/ui/button";
import { useToast } from "./hooks/use-toast";

export default function Home() {
  const { toast } = useToast();

  const functions = useJsonSharpener();
  const [inputText, setInputText] = useState(`{
    "guest": "Jason",
    "age": 25,
    "hungry": true,
    "order": ["pizza", "taco"],
    "payment": {
      "amount": 123.5,
      "card": true
    }
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

  function addToClipboard(val: string, desc: string): void {
    if (!window.isSecureContext) {
      toast({
        title: "An error occurred",
        description: "Using clipboard requires a secure origin",
        variant: "destructive",
      });
      return;
    }
    navigator.clipboard.writeText(val);
    toast({
      title: "Added to clipboard",
      description: desc,
    });
  }

  return (
    <div className="flex flex-col items-center h-full max-w-6xl min-w-full min-h-screen gap-16 p-8 pb-0 text-slate-300 bg-slate-900 justify-items-between sm:p-10 sm:pb-0">
      <main className="flex flex-col items-stretch w-full max-w-screen-lg row-start-2 gap-8">
        <div>
          <h1 className="text-2xl font-extrabold">JSON Sharpener</h1>
          <h2 className="font-semibold text-md">
            Powered by <span className="text-orange-700">Rust</span> and{" "}
            <span className="text-blue-700">WebAssembly</span>
          </h2>
        </div>
        <p
          className={
            errorText
              ? "bg-slate-800 p-3 font-semibold rounded-md text-amber-700"
              : "hidden"
          }>
          {errorText}
        </p>
        <div className="flex flex-col items-stretch justify-between flex-1 gap-6 md:flex-row md:gap-10">
          <div className="flex flex-col flex-auto gap-2">
            <div className="flex flex-row justify-between">
              <h3 className="text-xl font-bold">JSON</h3>
              <Button
                onClick={() => addToClipboard(inputText, "JSON code added")}
                className="px-2 rounded-md bg-slate-800">
                Copy
              </Button>
            </div>
            <Editor
              className="flex-1 border-2 !text-xs md:!text-sm lg:!text-base rounded border-slate-700 bg-slate-800"
              value={inputText}
              onValueChange={(code) => setInputText(code)}
              highlight={(code) =>
                highlight(code, Prism.languages.json, "json")
              }
              padding={10}
              style={{
                fontFamily: "monospace",
                fontSize: 14,
              }}
            />
          </div>
          <div className="flex flex-col flex-auto gap-2">
            <div className="flex flex-row justify-between">
              <h3 className="text-xl font-bold">C#</h3>
              <Button
                onClick={() => addToClipboard(outputText, "C# code added")}
                className="px-2 rounded-md bg-slate-800">
                Copy
              </Button>
            </div>
            <Editor
              className={
                !errorText
                  ? "border-2 !text-xs md:!text-sm lg:!text-base border-slate-700 rounded flex-1 bg-slate-800"
                  : "hidden"
              }
              value={outputText}
              onValueChange={(_) => {}}
              highlight={(code) =>
                Prism.highlight(code, Prism.languages.csharp, "csharp")
              }
              padding={10}
              style={{
                fontFamily: "monospace",
                fontSize: 14,
              }}
            />
          </div>
        </div>
      </main>
      <footer className="w-full py-2 mt-auto text-center text-white">
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
