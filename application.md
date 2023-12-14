# ink!jet

> This document will be part of the terms and conditions of your agreement and, therefore, needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines (except for the title)! Lines starting with a `>` (such as this one) should be removed. Please use markdown instead of HTML (e.g., `![](image.png)` instead of `<img>`).
>
> See the [Grants Program Process](https://github.com/w3f/Grants-Program/#pencil-process) on how to submit a proposal.

- **Team Name:** Parallel Polis
- **Payment Address:** In the case of fiat payment, please share your bank account privately with grants@web3.foundation via your contact email (see below) and enter the date when you shared the information with us (e.g. Fiat 24.12.1971, 11:59) here. Otherwise, provide the Polkadot (for USDC & USDT) payment address. Please also specify the currency. (e.g. 0x8920... (USDC))
- **[Level](https://github.com/w3f/Grants-Program/tree/master#level_slider-levels):** 3

> :exclamation: *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

This application is *not* in response to an RFP nor a follow-up grant.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tagline (one sentence summary).
- A brief description of your project.
- An indication of how your project relates to / integrates into Substrate / Polkadot / Kusama.
- An indication of why your team is interested in creating this project.

One liner: 

### Project Details

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

- Name of team leader
- Names of team members

### Contact

- **Contact Name:** Full name of the contact person in your team
- **Contact Email:** Contact email (e.g. john@duo.com)
- **Website:** Your website

### Legal Structure

- **Registered Address:** Address of your registered legal entity, if available. Please keep it in a single line. (e.g. High Street 1, London LK1 234, UK)
- **Registered Legal Entity:** Name of your registered legal entity, if available. (e.g. Duo Ltd.)

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past.

If anyone on your team has applied for a grant at the Web3 Foundation previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/<your_organisation>/<project_1>
- https://github.com/<your_organisation>/<project_2>

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/<team_member_1>
- https://github.com/<team_member_2>

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/<person_1>
- https://www.linkedin.com/<person_2>


## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/w3f/Grants-Program/tree/master/docs/RFPs) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Web3 Foundation,
- previous interface iterations, such as mock-ups and wireframes.

## Development Roadmap :nut_and_bolt:

This section should break the development roadmap down into milestones and deliverables. To assist you in defining it, we have created a document with examples for some grant categories [here](../docs/Support%20Docs/grant_guidelines_per_category.md). Since these will be part of the agreement, it helps to describe *the functionality we should expect in as much detail as possible*, plus how we can verify and test that functionality. Whenever milestones are delivered, we refer to this document to ensure that everything has been delivered as expected.

Below we provide an **example roadmap**. In the descriptions, it should be clear how your project is related to Substrate, Kusama or Polkadot. We *recommend* that teams structure their roadmap as 1 milestone ≈ 1 month.

> :exclamation: If any of your deliverables are based on somebody else's work, make sure you work and publish *under the terms of the license* of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Projects that submit other people's work without proper attribution will be immediately terminated.**

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
|       1. | Initial Prototype | Development of a basic LlamaIndex RAG system prototype integrated with GPT-4, using sentence embeddings. |
|       2. | Data Collection | Collection of a small set of `ink!` smart contracts for initial embedding and retrieval testing. Smart contracts will be converted from `.rs` files to `JSON`, with identifying metadata. |
|       3. | Loading and Embedding Pipeline | Pipeline for loading in `ink!` smart contracts as `JSON` files, and generating embeddings to build the initial vector database. |
|       4. | Vector Database | `Postgres` database, with `pgvector`, will be used to store embeddings from processed `ink!` smart contracts. |

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
|       1. | Data Expansion | Collection, categorization, and metadata tagging of `ink!` smart contracts to complete the dataset. |
|       2. | Embedding Model | Replace default sentence embedding. Implement a HuggingFace embedding model designated for code processing. Ensure model has substantial Rust code in training/fine-tuning data. |
|       3. | Vector Database | Update the `postgres` vector database to house the code-based embeddings of the complete dataset. |


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
|       4. | Final System Testing | System-wide testing of all features, ensuring full integration and operational stability. |
...


## Future Plans

Please include here

- how you intend to finance the project's long-term maintenance and development,
- how you intend to use, enhance, and promote your project in the short term, and
- the team's long-term plans and intentions in relation to it.

## Referral Program (optional) :moneybag:

You can find more information about the program [here](../README.md#moneybag-referral-program).

- **Referrer:** Name of the Polkadot Ambassador or GitHub account of the Web3 Foundation grantee
- **Payment Address:** Polkadot/Kusama (USDT/USDC) payment address. Please also specify the currency. (e.g. 0x8920... (USDT))

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.