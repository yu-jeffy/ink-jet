import React from 'react';
import './App.css';
import Header from './components/Header';
import CodeEditor from './components/CodeEditor';
import ChatHistory from './components/ChatHistory';

function App() {
  return (
    <body>
      <div className="application-container">
        <Header />
        <div className="editor-chat-container">
          <CodeEditor />
          <ChatHistory />
        </div>
        <div className="terminal-container">
          {/* Add terminal component here*/}
        </div>
      </div>
    </body>
  );
}

export default App;