from pdf_to_image import convert_pdf
from classification import classify_content
from ocrs import extract_content


def main(file_name: str):
    files_to_parse = [file_name]
    if 'pdf' in file_name:
        files = convert_pdf(file_name, file_name.replace('.pdf', ''))
        files_to_parse = files

    content = extract_content(files_to_parse)

    if content is not None:
        result = classify_content(content)
        print(f"{file_name} is classifed as: {result}")


if __name__ == '__main__':
    # file_name = 'docs/swissblatt24.pdf'
    file_name = 'docs/motel-one.pdf'
    main(file_name)



# TODO: 
# - save data to db, 
# - archive docs to folders, index content, categorize
# - run LLM in docker locally
