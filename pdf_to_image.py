from pdf2image import convert_from_path

def convert_pdf(pdf_file_path: str, target_file: str) -> [str]:
    pages = convert_from_path(pdf_file_path, 500)
    outs = []
    for count, page in enumerate(pages):
        title = f'{target_file}_{count}.jpg'
        outs.append(title)
        page.save(title, 'JPEG')
    return outs
