import PySimpleGUI as sg
import requests
import json

# Настройки LM Studio API (замените на свои)
LM_STUDIO_API_URL = "http://localhost:1234/v1/completions"  # Пример URL, проверьте в LM Studio
MODEL_NAME = "yandexgpt-5-lite-8b-instruct" # Замените на имя вашей модели

def get_lm_studio_response(prompt):
    headers = {"Content-Type": "application/json"}
    data = {
        "prompt": prompt,
        "model": MODEL_NAME,
        "max_tokens": 50,  # ограничение на размер ответа
    }
    try:
        response = requests.post(LM_STUDIO_API_URL, headers=headers, data=json.dumps(data))
        response.raise_for_status()  # Проверка на ошибки HTTP
        return response.json()['choices'][0]['text'] # Извлечение текста ответа
    except requests.exceptions.RequestException as e:
        return f"Ошибка при запросе к LM Studio API: {e}"

sg.theme('LightBlue2')  # Выберите тему оформления

layout = [
    [sg.Text("Введите запрос:")],
    [sg.InputText(key='-INPUT-', size=(60, 1))],
    [sg.Button("Запросить"), sg.Button("Выход")],
    [sg.Multiline(key='-OUTPUT-', size=(60, 10), disabled=True)]  # Для отображения ответа
]

window = sg.Window("LM Studio Integration", layout)

while True:
    event, values = window.read()
    if event == "Выход" or event == sg.WIN_CLOSED:
        break
    if event == "Запросить":
        prompt = values['-INPUT-']
        response = get_lm_studio_response(prompt)
        window['-OUTPUT-'].update(response)

window.close()
