This code was entirely generated using Antigravity using a mix of models (Gemini, Claude, GPT-OSS) available on free tier usage. The ping pong approach was used to refine and audit the code, where one model kept building and other two in parallel kept auditing and reviewing/hunting for bugs. 

The process took over 2 hours to complete, while my working knowledge of rust is limited at this time, after reading and reviewing the code i find no bugs/edge cases at this point in time. I will review this code again later after my working knowledge if rust has increased ovbiously.

What is interesting is that originally the prompt was given to build a encoder, but later the model decided itself to add a decoder in the same code. I blindly kept accepting the changes until there were no more cases/bugs left. 

The chat outputs will be added in a seperate file for review. Please find them under Prompts.md
