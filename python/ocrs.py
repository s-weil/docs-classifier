from typing import Optional
from easyocr import Reader

def __extract(reader: Reader, file_path: str) -> list[str]:
    result = reader.readtext(file_path)
    return [text for (_, text, _) in result]


def extract_content(files: list[str]) -> Optional[str]:
    if files is None or len(files) == 0:
        return None
    
    reader = Reader(['en'], gpu=True)
    contents_by_files = [ __extract(reader, file) for file in files ]

    result = [
        word
        for file_content in contents_by_files
        for word in file_content
    ]
    result = " ".join(result)

    return result