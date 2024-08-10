from typing import Optional
import requests as req


CLASSIFICATIONS = [
    "INSURANCE",
    "PAYMENT",
    "ORDER",
    "OFFER",
    "BOOKING",
    "WARANTY",
    "BANK",
]


def classify_content(content: str) -> Optional[str]:
    prompt = f"""Classify the content below with one of the posssible labels {', '.join(CLASSIFICATIONS)}. 
                 Only provide the label itself. The content is: {content}"""
    
    print(prompt)
    payload = {
        "model": "llama3",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": False
    }
    response = req.post('http://localhost:11434/api/chat', json=payload)

    results = response.json()
    if results is not None and results['message'] is not None:
        # TODO check if result is valid
        return results['message']['content']
    
    return None