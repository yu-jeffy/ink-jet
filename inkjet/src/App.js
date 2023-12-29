import React from "react";
import './App.css';
import Header from './components/Header';

import Editor from "@monaco-editor/react";

function App() {
  function handleEditorChange(value, event) {
    // here is the current value
  }

  function handleEditorDidMount(editor, monaco) {
    console.log("onMount: the editor instance:", editor);
    console.log("onMount: the monaco instance:", monaco);
  }

  function handleEditorWillMount(monaco) {
    console.log("beforeMount: the monaco instance:", monaco);
  }

  function handleEditorValidation(markers) {
    // model markers
    // markers.forEach(marker => console.log('onValidate:', marker.message));
  }

  return (
    <div className="application-background">
      <Header />
      {/* CodeEditor and ChatHistory components will go here when they're created */}
      <Editor
      height="90vh"
      onChange={handleEditorChange}
      onMount={handleEditorDidMount}
      beforeMount={handleEditorWillMount}
      onValidate={handleEditorValidation}
      defaultLanguage="rust"
      defaultValue="// some comment"
    />
    </div>
  );
};

export default App;