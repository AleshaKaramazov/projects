import json
import os
import urllib.request
import re

OLLAMA_URL = "http://localhost:11434/api/chat"
MODEL = "llama3"

SYSTEM_PROMPT = """
Язык ответа: ТОЛЬКО русский.

Ты — эксперт по алгоритмам, структурам данных и системному программированию.
Твоя специализация — анализ сложности, архитектура решений и низкоуровневая оптимизация.

Компетенции:
- Алгоритмы: Big-O, amortized и вероятностный анализ, графы, DP, greedy, divide & conquer.
- Структуры данных: массивы, списки, хеш-таблицы, деревья, графы, кучи, segment tree, DSU, B-деревья, Bloom filter.
- Оптимизация: cache locality, branch prediction, SIMD, аллокация памяти, lock-free, профилирование.
- Практика: C, C++, Rust, Python — с объяснением компромиссов.

Стиль ответа:
- Думай вслух и объясняй ход рассуждений.
- Сначала идея, затем структура, потом детали.
- Будь точным и лаконичным, без лишней воды.
- Задавай уточняющие вопросы, если требования неясны.
- Указывай на ошибки и неэффективности и предлагай альтернативы.
- Поддерживай, но не льсти.

Подход к задачам:
- Не начинай с кода. Сначала анализ.
- Рассматривай наивное решение и его ограничения.
- Определи bottleneck и компромиссы (time/space, сложность/производительность).
- Учитывай edge cases и предельные нагрузки.
- Цель — научить алгоритмическому мышлению, а не просто дать ответ.
"""

def send_request(messages):
    data = json.dumps({
        "model": MODEL,
        "messages": messages,
        "stream": False
    }).encode("utf-8")

    req = urllib.request.Request(
        OLLAMA_URL,
        data=data,
        headers={"Content-Type": "application/json"}
    )

    with urllib.request.urlopen(req) as response:
        result = json.loads(response.read())
        return result["message"]["content"]

def split_sentences(text):
    return re.split(r'(?<=[.!?])\s+', text.strip())

def main():
    print("(напишите 'все' или 'выход' для окончания диалога)")
    messages = [
        {"role": "system", "content": SYSTEM_PROMPT}
    ]
    #/Users/ct/desktop/neurotalks

    path = os.path.expanduser("/Users/ct/desktop/neurotalks/out.md")
    os.makedirs(os.path.dirname(path), exist_ok=True)

    with open(path, "w") as out:
        out.write("# Начало диалога\n")
        while True:
            try:
                user_input = input("ваш запрос:\n").strip()
            except EOFError:
                break

            if user_input.lower() in ("выход", "все"):
                break

            if not user_input:
                continue
            
            out.write(f"## Пользователь(Вы): \n");
            out.write(user_input + "\n");
            out.write("- - -\n")
            out.flush()
            messages.append({"role": "user", "content": user_input})
            try:
                answer = send_request(messages)
            except Exception as e:
                print(f"[error] {e}")
                continue
            messages.append({"role": "assistant", "content": answer})
            out.write("## Аи Ассистент: \n")
            for line in split_sentences(answer):
                out.write(line + "\n")
            out.write("\n---\n")
            out.flush()

            print("ответ получен и записан в файл");

if __name__ == "__main__":
    main()
