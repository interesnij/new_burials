https://docs.rs/distance/latest/distance/fn.levenshtein.html
https://tms-dev-blog.com/how-to-scrape-websites-with-rust-basic-example/#What_is_web_scraping

absolute_words = [
                "пизда",
                "блядь",
                "мудак",
                "мудил",
                "пидора",
                "дрочит",
                "залупа",
                "хуеть",
                "ебтвою",
                "заебись",
                "нахуйй",
                "похуйй",
                "нахерр",
                "похерр",
                "хуйня",
                "хуетень",
                "хуетаа",
                "хуёв",
                "охуев",
                "охуите",
                "нахуяч",
                "нехуйй",
                "похуйй",
                "нехуёв",
                "охуист",
                "хуйнутьь",
                "ахуячить",
                "тхуярить",
                "хуякнут",
                "дохуяч",
                "хуипан",
                "хуйлор",
                "уебало",
                "еблети",
                "ебатьь",
                "ебану́т",
                "ебальник",
                "ебаната",
                "ебанько",
                "ебанат",
                "выёбыва",
                "аебнуться",
                "поеботина",
                "поебень",
                "уёбищее",
                "заебала",
                "мозгоёб",
                "ебенев",
                "оебучий",
                "аебашить",
                "ебляя",
                "хуясебе",
                "хуйсни",
                "какогохуя",
                "схуяли",
                "хуюверт",
                "хуйнадо",
                "ебаныйв",
                "ебатьмозги",
                "оебучий",
                "аебашить",
                "ебляя",
                ]

relative_words = [
    "блять",
    "хуй",
    "хер",
    "хули",
    "бля",
]


def distance(a, b):
    "Вычисляет расстояние Левенштейна между a и b."
    n, m = len(a), len(b)
    if n > m:
        a, b = b, a
        n, m = m, n

    current_row = range(n + 1)
    for i in range(1, m + 1):
        previous_row, current_row = current_row, [i] + [0] * n
        for j in range(1, n + 1):
            add, delete, change = previous_row[j] + 1, current_row[j - 1] + 1, previous_row[j - 1]
            if a[j - 1] != b[i - 1]:
                change += 1
            current_row[j] = min(add, delete, change)

    return current_row[n]

def is_have_bad_words(text):
    lower_replace_text = text.lower().replace(" ", "")
    CLEANR = re.compile('<.*?>|&([a-z0-9]+|#[0-9]{1,6}|#x[0-9a-f]{1,6});')
    _text = re.sub(CLEANR, '', lower_replace_text)

    d = {'а' : ['а', 'a', '@'],
      'б' : ['б', '6', 'b'],
      'в' : ['в', 'b', 'v'],
      'г' : ['г', 'r', 'g'],
      'д' : ['д', 'd'],
      'е' : ['е', 'e'],
      'ё' : ['ё', 'e'],
      'ж' : ['ж', 'zh', '*'],
      'з' : ['з', '3', 'z'],
      'и' : ['и', 'u', 'i'],
      'й' : ['й', 'u', 'i'],
      'к' : ['к', 'k', 'i{', '|{'],
      'л' : ['л', 'l', 'ji'],
      'м' : ['м', 'm'],
      'н' : ['н', 'h', 'n'],
      'о' : ['о', 'o', '0'],
      'п' : ['п', 'n', 'p'],
      'р' : ['р', 'r', 'p'],
      'с' : ['с', 'c', 's'],
      'т' : ['т', 'm', 't'],
      'у' : ['у', 'y', 'u'],
      'ф' : ['ф', 'f'],
      'х' : ['х', 'x', 'h' , '}{'],
      'ц' : ['ц', 'c', 'u,'],
      'ч' : ['ч', 'ch'],
      'ш' : ['ш', 'sh'],
      'щ' : ['щ', 'sch'],
      'ь' : ['ь', 'b'],
      'ы' : ['ы', 'bi'],
      'ъ' : ['ъ'],
      'э' : ['э', 'e'],
      'ю' : ['ю', 'io'],
      'я' : ['я', 'ya']
    }
    for key, value in d.items():
        for letter in value:
            for phr in _text:
                if letter == phr:
                    _text = _text.replace(phr, key)

    for word in absolute_words:
        for part in range(len(_text)):
            fragment = _text[part: part+len(word)]
            if distance(fragment, word) <= len(word)*0.25:
                bad_text = "Исправьте слово: " + fragment + ", похожее на " + word
                raise PermissionDenied(bad_text)
    return False
