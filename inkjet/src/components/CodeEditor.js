import React from 'react';
import Editor from "@monaco-editor/react";
import './CodeEditor.css';

const CodeEditor = () => {
  function handleEditorChange(value, event) {
    // Handle the change of the editor's content
    console.log('Editor content: ', value);
  }

  function handleEditorDidMount(editor, monaco) {
    console.log("Editor mounted: the editor instance:", editor);
    console.log("Editor mounted: the monaco instance:", monaco);
  }

  function analyzeCode() {
    // Trigger analysis on the code currently in the editor
    console.log('Analyze button clicked');
  }

  return (
    <div className="code-editor-container">
      <h2 className="editor-title">Smart Contract Code</h2>
      <Editor
        height= "65vh"
        onChange={handleEditorChange}
        onMount={handleEditorDidMount}
        defaultLanguage="Rust"
        defaultValue="// Example ink! smart contract code"
        options={{
            automaticLayout: true,
            cursorStyle: 'line',
            cursorBlinking: 'blink',
            cursorWidth: 2, // Set the cursor width
            cursorSmoothCaretAnimation: true, // Enable smooth animation
         }}
      />
      <div className="editor-footer">
        <div className="editor-templates">
          <label htmlFor="templates" className="templates-label">Templates:</label>
          <select id="templates" className="templates-select">
            <option>Template 1</option>
            <option>Template 2</option>
            <option>Template 3</option>
          </select>
        </div>
        <button className="analyze-button" onClick={analyzeCode}>
          Analyze
        </button>
      </div>
    </div>
  );
};

export default CodeEditor;