"use client";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { Composer, TouchContextProvider } from "touchmacropad-ui/dist";
import {
  ChakraProvider,
  CheckboxCheckedChangeDetails,
  defaultSystem,
  Switch,
  Text,
} from "@chakra-ui/react";
import "touchmacropad-ui/dist/index.css";

function App() {
  const [isEditMode, setIsEditMode] = useState(false);
  const [isError, setIsError] = useState(false);
  const [keyPresses, setKeyPresses] = useState<string>("");

  async function greet(keys: string[]) {
    invoke("macros_command",{ keys });
  }

  const handleCheck = (details: CheckboxCheckedChangeDetails) => {
    if (isError) {
      return;
    }

    setIsEditMode(details.checked as boolean);
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    setKeyPresses((prev) => prev + e.key);
  };

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return (
    <ChakraProvider value={defaultSystem}>
      <Text>Press: {keyPresses}</Text>
      <div className="container">
        <Switch.Root
          checked={isEditMode}
          onCheckedChange={handleCheck}
          colorPalette="red"
          marginBottom="2rem"
        >
          <Switch.HiddenInput />
          <Switch.Control>
            <Switch.Thumb />
          </Switch.Control>
          <Switch.Label>edit mode</Switch.Label>
        </Switch.Root>
        <TouchContextProvider
          value={{
            handlers: {
              onWidgetEvent: (type, params) => {
                if (isEditMode) {
                  return;
                }

                if (type === 'macros') {
                  greet(params ?? []);
                }
              },
            },
          }}
        >
          <Composer
            isEditMode={isEditMode}
            gridConfig={{
              columns: 5,
              rows: 3,
            }}
            onError={setIsError}
            initialGridData={[
              {
                startColumn: 1,
                startRow: 1,
                size: {
                  width: 1,
                  height: 2,
                },
                color: "black",
                widget: "widgetWrapper",
                widgetParams: [],
              },
              {
                startColumn: 2,
                startRow: 1,
                size: {
                  width: 1,
                  height: 2,
                },
                color: "yellow",
                widget: "widgetWrapper",
                widgetParams: [],
              },
              {
                startColumn: 3,
                startRow: 1,
                size: {
                  width: 1,
                  height: 2,
                },
                color: "purple",
                widget: "widgetWrapper",
                widgetParams: [],
              },
              {
                startColumn: 4,
                startRow: 1,
                size: {
                  width: 1,
                  height: 1,
                },
                color: "orange",
                widget: "macros",
                widgetParams: {
                  imageHref:
                    "https://cdn.prod.website-files.com/6257adef93867e50d84d30e2/66e278299a53f5bf88615e90_Symbol.svg",
                  title: undefined,
                  keys: ["window", "shift", "z"],
                },
              },
            ]}
          />
        </TouchContextProvider>
      </div>
    </ChakraProvider>
  );
}

export default App;
