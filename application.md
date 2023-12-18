# ink!jet

> This document will be part of the terms and conditions of your agreement and, therefore, needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines (except for the title)! Lines starting with a `>` (such as this one) should be removed. Please use markdown instead of HTML (e.g., `![](image.png)` instead of `<img>`).
>
> See the [Grants Program Process](https://github.com/w3f/Grants-Program/#pencil-process) on how to submit a proposal.

- **Team Name:** Parallel Polis
- **Payment Address:** In the case of fiat payment, please share your bank account privately with grants@web3.foundation via your contact email (see below) and enter the date when you shared the information with us (e.g. Fiat 24.12.1971, 11:59) here. Otherwise, provide the Polkadot (for USDC & USDT) payment address. Please also specify the currency. (e.g. 0x8920... (USDC))
- **[Level](https://github.com/w3f/Grants-Program/tree/master#level_slider-levels):** 2

> :exclamation: *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

This application is *not* in response to an RFP nor a follow-up grant.

### Overview

**One liner:** Platform for Generative AI Assisted Smart Contract Development with Retrieval Augmented Generation

`Ink!jet` is a platform designed to use augmented generative AI with the intent of improving the development lifecycle of `ink!` smart contracts within the Polkadot ecosystem. Recognizing the technically intricate nature of smart contracts and the high level of expertise they demand, our project aims to simplify these complexities, thereby democratizing the creation process.

Existing generative AI have limitations in both the amount of `Rust` and `ink!` code in their training data. Our platform uses a retrieval-augmented generation pipeline with datasets of existing `ink!` smart contracts to bridge this knowledge gap. Injecting vectorstore retrieved code into prompts, this system utilizes in-context learning to improve response quality. Our goal is to enhance the productivity of existing developers through bootstrapping and assisted code interation, while simultaneously lowering the barrier of entry for new developers.

Our team's motivation for the project is twofold. First, we are supporters of decentralized technology and its potential to redefine the digital landscape. We believe that the future of blockchain depends on cross-chain interoperability, which places Polkadot at the center of this emerging need. Second, we are deeply intrigued by the potential of generative AI in software engineering. Our aim is to push the boundaries of what these AI models can achieve in novel contexts, such as the `ink!` programming language, and to explore solutions that empower developers to build within the decentralized ecosystem.

### Project Details

#### UI Mockup:
![ui-mockup](https://jyu.llc/inkjet_mockup.png)

#### Data Model / Architecture



#### Stack

We will be using the following technologies:
- Python for RAG-LLM pipeline
 - LlamaIndex and LangChain libraries for data loading, processing, embedding
 - LlamaIndex and LangChain libraries for vectorstore retrieval and LLM interaction with retrieval results
- Pinecone for Vectorstore
- OpenAI text-embedding-ada-002 Model for Embeddings
- OpenAI for LLM (GPT-4-32k, GPT-4-1106-preview)
- ink!/Rust for Smart Contracts
- React.js for Front-End Application
- Docker for Containerization
- Vercel for Hosting

#### Relevant Work

Please see Development Status section.

#### What this project is NOT

We are not providing a for-profit product, and all usage will be free to any developers. The system and datasets will also be public, so developers can choose to host their own version of the system, with modifications if they please.

We are not providing full automation nor replacement for existing developers, this tool is designed to enhance the developement cycle and increase efficiency.

We are not claiming that this system is without fault. Though the system is aimed at mitigating errors and vulnerabilities, there is a degree of inherent randomness when using LLMs for code generation. We will provide stringent disclaimers and advise to users to rigorously test their code before deployment, and advocate for contract auditing.

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is *not* or will *not* provide or implement
  - This is a place for you to manage expectations and clarify any limitations that might not be obvious


Things that shouldn’t be part of the application (see also our [FAQ](../docs/faq.md)):

- The (future) tokenomics of your project
- For non-infrastructure projects—deployment and hosting costs, maintenance or audits
- Business-oriented activities (marketing, business planning), events or outreach

### Ecosystem Fit

Help us locate your project in the Polkadot/Substrate/Kusama landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- What need(s) does your project meet?
- How did you identify these needs? Please provide evidence in the form of (scientific) articles, forum discussions, case studies, or raw data.
- Are there any other projects similar to yours in the Substrate / Polkadot / Kusama ecosystem?
  - If so, how is your project different? Please identify and assess any projects addressing the same need and explain how your project is distinct. Feel free to include applicable research data, statistics, or metrics.
  - If not, please indicate why such a project might not have been possible, successful, or attempted. 
- Are there any projects similar to yours in related ecosystems? 

## Team :busts_in_silhouette:

> Please note that the data provided in this section is for administrative and informational purposes only. All beneficiaries of a grant must also be listed in the KYC/KYB process during the application phase. See our [FAQ](https://grants.web3.foundation/docs/faq#what-is-kyckyb-and-why-do-i-have-to-provide-this-data) for more info.

### Team members

Jeffy Yu
Maximilian Huber
Kevin Tang

### Contact

- **Contact Name:** Jeffy Yu
- **Contact Email:** jeff.yu@parallelpolis.llc
- **Website:** jyu.llc

### Legal Structure

- **Registered Address:** 656 W Nopal Ave Mesa, Arizona 85210
- **Registered Legal Entity:** Parallel Polis

### Team's Experience

We are a research collective conducting open source research on areas in blockchain, machine learning, artificial intelligence, and general software engineering.

Our team has an academic background in computer science, and work experience in blockchain development, machine learning, and full stack development. We are looking to bring our skills towards a project that can improve accessibility of the Polkadot developer ecosystem.

Jeffy Yu - Former full stack blockchain engineer at Paystand, a B2B payments company anchoring invoice records on the Ethereum blockchain. A member of the R&D team at Paystand, built a payment network in Rust supported by the Lightning Network to migrate invoicing towards decentralized solutions. Former full stack engineer at Lirn, a decentralized learning platform utilizing ERC-1155 tokens as educational certificates. Research Scientist at Parallel Polis, performing open-sourced technical studies on blockchain and artificial intelligence. Publications can be found [here](https://scholar.google.com/citations?user=zy7ZWqYAAAAJ&hl=en).

Maximilian Huber: Former full stack engineer at CollX Inc., experienced in building RAG retrieval systems and deploying generative AI for specific use cases.

Kevin Tang: Full stack engineer with primary experience in JavaScript and React.js, as well as experience in iOS-development at Priceline.com. Experience developing and maintaining systems in active deployment.

### Team Code Repos

- (https://github.com/yu-jeffy/audit.me)[https://github.com/yu-jeffy/audit.me]
- (https://github.com/yu-jeffy/GreedLlama)[https://github.com/yu-jeffy/GreedLlama]
- (https://github.com/yu-jeffy/PyArb)[https://github.com/yu-jeffy/PyArb]
- (https://github.com/yu-jeffy/gpt-btc)[https://github.com/yu-jeffy/gpt-btc]

#### GitHub accounts of all team members

- (https://github.com/yu-jeffy/)[https://github.com/yu-jeffy/]
- (https://github.com/MaxHuber88)[https://github.com/MaxHuber88]
- (https://github.com/tangk01)[https://github.com/tangk01]

### Team LinkedIn Profiles

- (https://www.linkedin.com/in/jeffyyu/)[https://www.linkedin.com/in/jeffyyu/]
- (https://www.linkedin.com/in/max-huber88/)[https://www.linkedin.com/in/max-huber88/]
- (https://www.linkedin.com/in/tangk01/)[https://www.linkedin.com/in/tangk01/]


## Development Status :open_book:

We have done extensive research on existing work and the feasibility of the system. Here are our findings:

To our knowledge, there are currently three publications assessing the efficacy of LLMs for smart contract auto-completion and generation.
- (https://arxiv.org/abs/2308.02955)[https://arxiv.org/abs/2308.02955]

This publication uses the base model without additional learning, and finds limited success, with security bugs and errors being frequent. We look to address this by providing additional context through our RAG system, which provides GPT-4 examples of accurate code to reference.

- (https://arxiv.org/abs/2311.10388)[https://arxiv.org/abs/2311.10388]
This publication utilizes in-context learning with a retrieval system, similar to our approach. They focus solely on generating code comments, not on code itself. They find improved efficacy, which is promising as the design is similar to our system. 

However, their design is distinct from ours as they utilize CodeBert to create the embeddings, which has no Rust code in its training data. This makes it incompatible with ink! smart contracts. They also utilize an older model of GPT, GPT-3.5-turbo, which has performed significantly worse than GPT-4 on coding benchmarks. 

We aim to create an alternative system and replicate their efficacy improvements, using an embedding model that supports Rust, an more recent GPT model, and extend functionality beyond code comment generation to smart contract code generation.

- (https://arxiv.org/abs/2309.09826)[https://arxiv.org/abs/2309.09826]
This publication fine-tunes a GPT-J model on 2 million smart contracts, and tests if performance increases on writing code without security issues. While their methodology involved directly fine-tuning the model and updating the weights, and ours focuses on adding context through retrieval, the underlying concept of providing learning examples to improve performance remains similar. After the original fine-tuning, they find that insecure code was an issue in up to 70% of generations. After additional fine-tuning on vulnerable examples with vulnerability-constrained decoding, they were able to avoid insecure code generation up to 67% of the time. We have taken these results into consideration, and will implement labeled vulnerable examples in our dataset. This allows our system to identify vulnerabilities, and also avoid generation of them in produced code.

In terms of related work, we have (previous work)[https://github.com/yu-jeffy/audit.me] studying the efficacy of vulnerability testing through RAG integrated LLMs on Ethereum Solidity smart contracts. In this study, we built a LangChain RAG-LLM pipeline, and created a vectorstore of 830 vulnerable smart contracts for retrieval. Results were promising, showing a 1.5x increased efficacy compared to (current literature)[https://arxiv.org/abs/2306.12338]. Using this as a proof of concept of RAG-LLM with smart contract data, we look to rebuild the pipeline towards the use case of authoring ink! smart contracts. We will construct a new pipeline from scratch to utilize LlamaIndex, change the data processing and embedding, and implement a much broader ink! smart contract dataset.


## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 5 months
- **Full-Time Equivalent (FTE):** 3 FTE
- **Total Costs:** 30,000 USD

### Milestone 1 — Prototype, Initial RAG System and Data Pipeline

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 6,000 USD

|  Number  | Deliverable | Specification |
| -------: | ----------- | ------------- |
|  **0a.** | License | Open-sourced under Apache 2.0. |
|  **0b.** | Documentation | Code comments. Documentation for the prototype architecture and the setup process, explaining how a user can run a local instance of the prototype RAG system with our initial data. |
|  **0c.** | Testing Guide | Unit tests will be provided for the prototype, with a testing guide explaining how to run them. |
|  **0d.** | Docker | We will include a `Dockerfile` that enables easy deployment and testing of the RAG system. |
|       1. | Initial Prototype | Development of a basic LlamaIndex RAG system prototype integrated with `GPT-4`, using sentence embeddings. User can interact with the pipeline through the command line, interfacing with `GPT-4` with fetched documents from `Pinecone`|
|       2. | Data Collection | Collection of a small set of `ink!` smart contracts for initial embedding and retrieval testing. Smart contracts will be converted from `.rs` files to `JSON`, with identifying metadata. |
|       3. | Loading and Embedding Pipeline | Pipeline for loading in `ink!` smart contracts as `JSON` files, and generating embeddings to build the initial vector database. |
|       4. | Vector Database | `Pinecone` database will be used to store embeddings from processed `ink!` smart contracts. |

### Milestone 2 — Smart Contract Dataset, Embeddings for Vector Database

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 6,000 USD

|  Number  | Deliverable | Specification |
| -------: | ----------- | ------------- |
|  **0a.** | License | Open-sourced under Apache 2.0. |
|  **0b.** | Documentation | Code comments. Documentation on the process vectorization of newly collected smart contracts and updating the vector database. |
|  **0c.** | Testing Guide | Unit tests for changes to loading and vectorization. Testing guide included for running these tests. |
|  **0d.** | Docker | `Dockerfile` updated to reflect any changes in RAG system and data pipeline deployment. |
|       1. | Data Expansion | Collection, categorization, and metadata tagging of `ink!` smart contracts to complete the dataset. Rigorous code commentation in the smart contracts to provide semantic context before vectorization. |
|       2. | Embedding Model | Replace default sentence embedding. Implement OpenAI's `text-embedding-ada-002` model, compatible with natural language and code processing. |
|       3. | Vector Database | Update the `Pinecone` vector database to house the code-based embeddings of the complete dataset. |


### Milestone 3 — RAG System Integration

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 6,000 USD

|  Number  | Deliverable | Specification |
| -------: | ----------- | ------------- |
|  **0a.** | License | Open-sourced under Apache 2.0. |
|  **0b.** | Documentation | Code comments. Documentation provided for the integration process, and usage instructions provided for the updated RAG system. |
|  **0c.** | Testing Guide | Unit tests updated to reflect changes in RAG system. Guide for testing the updated RAG system with various queries included. |
|  **0d.** | Docker | `Dockerfile` updated with any changes in deployment. |
|       1. | RAG Integration | Integration of the new dataset and vector database with the prototype RAG system for improved retrieval. |
|       2. | Query Handling | Updated query parsing and processing using new embedding model to improve smart contract code retrieval. |
|       3. | System Testing | Testing of RAG retrieval functionality with the updated dataset. Testing of output quality of RAG system. |

### Milestone 4 — UI/UX Development and Front-End Implementation

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 6,000 USD

|  Number  | Deliverable | Specification |
| -------: | ----------- | ------------- |
|  **0a.** | License | Open-sourced under Apache 2.0. |
|  **0b.** | Documentation | Code comments. UI/UX design documentation and front-end codebase documentation, including setup and deployment procedures. |
|  **0c.** | Testing Guide | Guide on how to run tests on the `React.js` application. |
|  **0d.** | Docker | Updated `Dockerfile` to include the `React.js` application and associated dependencies. |
|       1. | UI/UX Design | Design and development of user interface for the RAG system. |
|       2. | React.js Application | Development of `React.js` front-end for the RAG system. Code implementation of the UI/UX design. |
|       3. | Front-End Testing | Comprehensive testing to ensure the front-end application is responsive and stable. |

### Milestone 5 — Additional Features

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 6,000 USD

|  Number  | Deliverable | Specification |
| -------: | ----------- | ------------- |
|  **0a.** | License | Open-sourced under Apache 2.0. |
|  **0b.** | Documentation | Code comments. Documentation of additional features, including use cases and integration into the existing system. |
|  **0c.** | Testing Guide | Updated testing guide that includes all features of the application. |
|  **0d.** | Docker | Finalized `Dockerfile` for the complete system with all features included. |
|  **0e.** | Article | We will publish an **article** that showcases the development process, application use cases, application demo, and usage of the grant throughout the project cycle. |
|       1. | UI/UX Updates | Updated `React.js` to serve additional features in the application. |
|       2. | Templates | Development of a template system for users to easily bootstrap building smart contracts. User can pick from a list of common use cases, and be provided a template smart contract to start with. |
|       3. | Smart Contract Analysis | A feature to break down smart contracts into chunks, scanning each for functionality and errors, and suggest improvements or fixes. Contract will be broken down into parts, such as its functions, and each will be analyzed by the LLM. |
|       4. | Final Application Testing | System-wide testing of all features, ensuring full integration and operational stability. |


## Future Plans

- We plan to set aside a significant portion of the grant funding towards ongoing OpenAI API costs and other hosting costs.
- We will secure additional funding as needed for these ongoing costs if there is a substantial demand on the platform.
- We intend to deploy for the public to use for all developers.
- We look to add additional features to the platform, including Solang support and test deployment capabilities.
- We look to continuously improve the vectorstore, through gathering of more smart contract examples and vulnerability avoidance.
- We look to improve the vector search, such as through improving semantic relations with specific code comment styles.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Web3 Foundation Website
