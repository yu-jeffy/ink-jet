import React from 'react';
import './App.css';
import Header from './components/Header';
import CodeEditor from './components/CodeEditor';

function App() {
  return (
    <div className="application-background">
      <Header />
      <CodeEditor />
      {/* ChatHistory components will go here when it's created */}
    </div>
  );
}

export default App;