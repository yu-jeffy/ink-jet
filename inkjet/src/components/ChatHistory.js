import React from 'react';
import './ChatHistory.css';

const ChatHistory = () => {
  // Placeholder chat messages for demonstration
  const chatMessages = [
    { type: "gpt-message", content: "Hello, I'm GPT-4. How can I assist you?" },
    { type: "user-message", content: "Can you provide an example of a smart contract?" },
    { type: "gpt-message", content: "Certainly! Here's a simple ink! smart contract example." },
  ];

  return (
    <div className="chat-container">
      <h2 className="chat-history-title">Chat History</h2>
      <div className="chat-history">
        {chatMessages.map((message, index) => (
          <div key={index} className={`chat-message ${message.type}`}>
            {message.content}
          </div>
        ))}
      </div>
      <div className="chat-inputs">
        {/* Add temperature and top-k input fields here, skipping for brevity */}
      </div>
      <div className="text-input-container">
        <label htmlFor="text-input" className="input-label">Text Input:</label>
        <input
          type="text"
          id="text-input"
          placeholder="Enter your message"
          className="text-input"
        />
      </div>
    </div>
  );
};

export default ChatHistory;